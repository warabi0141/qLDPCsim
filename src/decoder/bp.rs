use crate::math::sparse_matrix::BinarySparseMatrix;

use rand::rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
pub enum BpMethod {
    ProductSum = 0,
    MinimumSum = 1,
}

#[derive(PartialEq, Clone, Copy)]
pub enum BpSchedule {
    Serial = 0,
    Parallel = 1,
    SerialRelative = 2,
}
// 行列のエントリー（エッジ）を表す構造体の想定
pub struct BpEntry {
    pub row_index: usize,
    pub col_index: usize,
    pub bit_to_check_msg: f64,
    pub check_to_bit_msg: f64,
}

// 疎行列構造体の想定（C++のBpSparse相当）
pub struct BpSparse {
    parity_check_matrix: BinarySparseMatrix,
    entries: HashMap<(usize, usize), BpEntry>, // (row, col) -> BpEntry
}
impl BpSparse {
    pub fn new(parity_check_matrix: BinarySparseMatrix) -> Self {
        let mut entries = HashMap::new();

        for row_idx in 0..parity_check_matrix.rows() {
            for &col_idx in parity_check_matrix.nonzero_cols(row_idx) {
                entries.insert(
                    (row_idx, col_idx),
                    BpEntry {
                        row_index: row_idx,
                        col_index: col_idx,
                        bit_to_check_msg: 0.0,
                        check_to_bit_msg: 0.0,
                    },
                );
            }
        }

        Self {
            parity_check_matrix,
            entries,
        }
    }

    pub fn parity_check_matrix(&self) -> &BinarySparseMatrix {
        &self.parity_check_matrix
    }

    pub fn iterate_row(&self, row: usize) -> Vec<&BpEntry> {
        let cols = self.parity_check_matrix.nonzero_cols(row).to_vec();
        cols.iter()
            .filter_map(|&col| self.entries.get(&(row, col)))
            .collect()
    }

    pub fn iterate_row_mut(&mut self, row: usize) -> Vec<&mut BpEntry> {
        let cols = self.parity_check_matrix.nonzero_cols(row).to_vec();
        let entries_ptr: Vec<*mut BpEntry> = cols
            .iter()
            .filter_map(|&col| self.entries.get_mut(&(row, col)).map(|e| e as *mut BpEntry))
            .collect();
        // 安全に可変参照を返す
        entries_ptr
            .into_iter()
            .map(|ptr| unsafe { &mut *ptr })
            .collect()
    }

    pub fn reverse_iterate_row_mut(&mut self, row: usize) -> Vec<&mut BpEntry> {
        let mut cols = self.parity_check_matrix.nonzero_cols(row).to_vec();
        cols.reverse();
        let entries_ptr: Vec<*mut BpEntry> = cols
            .iter()
            .filter_map(|&col| self.entries.get_mut(&(row, col)).map(|e| e as *mut BpEntry))
            .collect();
        // 安全に可変参照を返す
        entries_ptr
            .into_iter()
            .map(|ptr| unsafe { &mut *ptr })
            .collect()
    }

    pub fn iterate_column(&self, col: usize) -> Vec<&BpEntry> {
        let rows = self.parity_check_matrix.nonzero_rows(col).to_vec();
        rows.iter()
            .filter_map(|&row| self.entries.get(&(row, col)))
            .collect()
    }

    pub fn iterate_column_mut(&mut self, col: usize) -> Vec<&mut BpEntry> {
        let rows = self.parity_check_matrix.nonzero_rows(col).to_vec();
        let entries_ptr: Vec<*mut BpEntry> = rows
            .iter()
            .filter_map(|&row| {
                self.entries
                    .get(&(row, col))
                    .map(|e| e as *const BpEntry as *mut BpEntry)
            })
            .collect();
        // 安全に可変参照を返す
        entries_ptr
            .into_iter()
            .map(|ptr| unsafe { &mut *ptr })
            .collect()
    }

    pub fn reverse_iterate_column_mut(&mut self, col: usize) -> Vec<&mut BpEntry> {
        let mut rows = self.parity_check_matrix.nonzero_rows(col).to_vec();
        rows.reverse();
        let entries_ptr: Vec<*mut BpEntry> = rows
            .iter()
            .filter_map(|&row| {
                self.entries
                    .get(&(row, col))
                    .map(|e| e as *const BpEntry as *mut BpEntry)
            })
            .collect();
        // 安全に可変参照を返す
        entries_ptr
            .into_iter()
            .map(|ptr| unsafe { &mut *ptr })
            .collect()
    }

    pub fn update_edge_msg<F>(&mut self, row: usize, col: usize, update_fn: F)
    where
        F: FnOnce(&mut BpEntry),
    {
        if let Some(entry) = self.entries.get_mut(&(row, col)) {
            update_fn(entry);
        }
    }
}

pub struct BpDecoder {
    pcm: BpSparse,
    bit_count: usize,
    bp_method: BpMethod,
    schedule: BpSchedule,
    maximum_iterations: usize,
    ms_scaling_factor: f64,
    random_serial_schedule: bool,
    channel_probabilities: Vec<f64>,
    initial_log_prob_ratios: Vec<f64>,
    log_prob_ratios: Vec<f64>,
    decoding: Vec<u8>,
    candidate_syndrome: Vec<u8>,
    converge: bool,
    iterations: usize,
    serial_schedule_order: Vec<usize>,
    // rng_list_shuffle: rand::seq::SliceRandom, // 乱数シャッフル用
}

// Reference: LDPC: Python tools for low density parity check codes
// Authors: Roffe, Joschka
// URL: https://github.com/quantumgizmos/ldpc
// Year: 2022

impl BpDecoder {
    pub fn from_pcm(
        pcm: BinarySparseMatrix,
        bp_method: BpMethod,
        schedule: BpSchedule,
        max_iterations: usize,
        ms_scaling_factor: f64,
        random_serial_schedule: bool,
        channel_probabilities: Vec<f64>,
    ) -> Self {
        let bit_count = pcm.cols();
        let initial_log_prob_ratios = vec![0.0; bit_count];
        let log_prob_ratios = vec![0.0; bit_count];
        let decoding = vec![0; bit_count];
        let candidate_syndrome = vec![0; pcm.rows()];
        let serial_schedule_order: Vec<usize> = (0..bit_count).collect();

        Self {
            pcm: BpSparse::new(pcm),
            bit_count,
            bp_method,
            schedule,
            maximum_iterations: max_iterations,
            ms_scaling_factor,
            random_serial_schedule,
            channel_probabilities,
            initial_log_prob_ratios,
            log_prob_ratios,
            decoding,
            candidate_syndrome,
            converge: false,
            iterations: 0,
            serial_schedule_order,
        }
    }

    /// チャネル確率から初期対数尤度比(LLR)を計算し、変数ノードからのメッセージを初期化します。
    pub fn initialise_log_domain_bp(&mut self) {
        for i in 0..self.bit_count {
            // LLR = ln((1-p)/p)
            let p = self.channel_probabilities[i];
            self.initial_log_prob_ratios[i] = ((1.0 - p) / p).ln();

            // 変数ノードからチェックノードへの初期メッセージを設定
            for entry in self.pcm.iterate_column_mut(i) {
                entry.bit_to_check_msg = self.initial_log_prob_ratios[i];
            }
        }
    }

    pub fn decode(&mut self, syndrome: &Vec<u8>) -> Vec<u8> {
        if self.schedule == BpSchedule::Parallel {
            self.bp_decode_parallel(syndrome)
        } else {
            self.bp_decode_serial(syndrome)
        }
    }

    /// C++: bp_decode_parallel
    /// 並列スケジュールでのBP復号（積和法または最小和法）
    fn bp_decode_parallel(&mut self, syndrome: &Vec<u8>) -> Vec<u8> {
        let check_count = self.pcm.parity_check_matrix().rows();
        self.converge = false;
        self.initialise_log_domain_bp();

        for it in 1..=self.maximum_iterations {
            // --- チェックノード更新 (Check Node Update) ---
            if self.bp_method == BpMethod::ProductSum {
                // Product Sum (Tanh rule)
                // Forward-Backward アルゴリズムを使って、自分自身以外の積を計算
                for i in 0..check_count {
                    self.candidate_syndrome[i] = 0;

                    // Forward pass: 左からの積を計算して check_to_bit_msg に一時保存
                    let mut temp = 1.0;
                    for entry in self.pcm.iterate_row_mut(i) {
                        entry.check_to_bit_msg = temp;
                        temp *= (entry.bit_to_check_msg / 2.0).tanh();
                    }

                    // Backward pass: 右からの積を計算し、Forwardの結果と結合
                    temp = 1.0;
                    for entry in self.pcm.reverse_iterate_row_mut(i) {
                        // 逆順イテレータ
                        entry.check_to_bit_msg *= temp; // Left * Right

                        let message_sign = if syndrome[i] != 0 { -1.0 } else { 1.0 };
                        // 2 * atanh(x) = ln((1+x)/(1-x))
                        entry.check_to_bit_msg = message_sign
                            * ((1.0 + entry.check_to_bit_msg) / (1.0 - entry.check_to_bit_msg))
                                .ln();

                        // 次のイテレーション用にRight積を更新
                        temp *= (entry.bit_to_check_msg / 2.0).tanh();
                    }
                }
            } else if self.bp_method == BpMethod::MinimumSum {
                // Minimum Sum
                // アルファスケーリング係数の決定
                let alpha = if self.ms_scaling_factor == 0.0 {
                    1.0 - 2.0_f64.powf(-1.0 * it as f64)
                } else {
                    self.ms_scaling_factor
                };

                for i in 0..check_count {
                    self.candidate_syndrome[i] = 0;
                    let mut total_sgn = syndrome[i] as i32;

                    // Forward pass: グローバルな最小値を探索しつつ、符号をカウント
                    // 注: bp.hppの実装ではForward-Backwardで自分以外の最小値を厳密に求めている

                    // Forward loop
                    let mut temp = f64::MAX;
                    for entry in self.pcm.iterate_row_mut(i) {
                        if entry.bit_to_check_msg <= 0.0 {
                            total_sgn += 1;
                        }
                        // entry.check_to_bit_msg に現在の「左側の最小値」を保持
                        entry.check_to_bit_msg = temp;
                        let abs_msg = entry.bit_to_check_msg.abs();
                        if abs_msg < temp {
                            temp = abs_msg;
                        }
                    }

                    // Backward loop
                    temp = f64::MAX;
                    for entry in self.pcm.reverse_iterate_row_mut(i) {
                        // 自分自身を符号カウントから除外する
                        let mut sgn = total_sgn;
                        if entry.bit_to_check_msg <= 0.0 {
                            sgn += 1; // トータルに含まれているので、+1するとmod 2でキャンセルされる効果
                        }

                        // Right側の最小値(temp)とLeft側の最小値(entry.check_to_bit_msg)を比較
                        // entry.check_to_bit_msg には最終的に「自分以外」の最小値が入る
                        if temp < entry.check_to_bit_msg {
                            entry.check_to_bit_msg = temp;
                        }

                        let message_sign = if sgn % 2 == 0 { 1.0 } else { -1.0 };
                        entry.check_to_bit_msg *= message_sign * alpha;

                        // 次のイテレーション用にRight最小値を更新
                        let abs_msg = entry.bit_to_check_msg.abs();
                        if abs_msg < temp {
                            temp = abs_msg;
                        }
                    }
                }
            }

            // --- 変数ノード更新 (Bit Node Update) ---
            // log probability ratios の計算
            for i in 0..self.bit_count {
                let mut temp = self.initial_log_prob_ratios[i];

                // 列方向のメッセージを合算（入力LLR + Σ check_to_bit）
                for entry in self.pcm.iterate_column_mut(i) {
                    entry.bit_to_check_msg = temp; // Forward pass的に保存しているが、次のループで上書きされる
                    temp += entry.check_to_bit_msg;
                }

                self.log_prob_ratios[i] = temp;

                // 硬判定
                if temp <= 0.0 {
                    self.decoding[i] = 1;
                    // 候補シンドロームの更新（フリップ）
                    for entry in self.pcm.iterate_column(i) {
                        self.candidate_syndrome[entry.row_index] ^= 1;
                    }
                } else {
                    self.decoding[i] = 0;
                }
            }

            // 収束判定
            if self.candidate_syndrome == *syndrome {
                self.converge = true;
            }
            self.iterations = it;

            if self.converge {
                return self.decoding.clone();
            }

            // 次のイテレーションのために bit_to_check メッセージを計算
            // sum(all) - msg_from_check
            for i in 0..self.bit_count {
                let mut temp = 0.0;
                // Reverse iterate してBackward sumを計算しつつ、Forward sumと合わせるのが効率的だが
                // bp.hppの実装(source 8, line 427)は単純に累積加算しているように見える（Forward-Backwardを意識）

                // C++実装:
                // for (auto &e: reverse_iterate_column(i)) {
                //    e.bit_to_check_msg += temp; // e.bit_to_check_msgには既にForwardからの部分和が入っている前提か？
                //    temp += e.check_to_bit_msg;
                // }
                // 注: line 410で `e.bit_to_check_msg = temp` (left sum) しているので、
                // ここで right sum を足せば「自分以外」の和になる。

                for entry in self.pcm.reverse_iterate_column_mut(i) {
                    entry.bit_to_check_msg += temp;
                    temp += entry.check_to_bit_msg;
                }
            }
        }

        self.decoding.clone()
    }

    /// 逐次スケジュールでのBP復号
    fn bp_decode_serial(&mut self, syndrome: &Vec<u8>) -> Vec<u8> {
        self.converge = false;
        // BPの初期化（LLRの計算とメッセージの初期化）
        self.initialise_log_domain_bp();

        // メイン反復ループ
        for it in 1..=self.maximum_iterations {
            // 1. Minimum Sum用のスケーリング係数(alpha)の計算
            let alpha = if self.ms_scaling_factor == 0.0 {
                1.0 - 2.0_f64.powf(-1.0 * it as f64)
            } else {
                self.ms_scaling_factor
            };

            // 2. スケジュールの更新（ランダム or 相対的信頼度順）
            if self.random_serial_schedule {
                let mut rng = rng();
                self.serial_schedule_order.shuffle(&mut rng);
            } else if self.schedule == BpSchedule::SerialRelative {
                // LLRの絶対値（信頼度）に基づいてソート
                let channel_probs = &self.channel_probabilities;
                let llrs = &self.log_prob_ratios;

                self.serial_schedule_order.sort_by(|&a, &b| {
                    let idx_a = a as usize;
                    let idx_b = b as usize;

                    let val_a = if it == 1 {
                        let p = channel_probs[idx_a];
                        ((1.0 - p) / p).ln().abs()
                    } else {
                        llrs[idx_a].abs()
                    };

                    let val_b = if it == 1 {
                        let p = channel_probs[idx_b];
                        ((1.0 - p) / p).ln().abs()
                    } else {
                        llrs[idx_b].abs()
                    };

                    // 降順ソート（信頼度が高い順）
                    val_b.partial_cmp(&val_a).unwrap_or(Ordering::Equal)
                });
            }

            // 3. ビットごとの逐次更新ループ
            for &bit_index_i32 in &self.serial_schedule_order {
                let bit_index = bit_index_i32 as usize;

                // チャネル値でLLRをリセット
                let p = self.channel_probabilities[bit_index];
                self.log_prob_ratios[bit_index] = ((1.0 - p) / p).ln();

                // ---------------------------------------------------------
                // Step A: チェックノードからのメッセージを計算し、LLRを更新
                // ---------------------------------------------------------

                // Rustの借用規則回避のため、インデックスを収集してから処理
                // self.pcm.iterate_column(bit_index) に相当
                let connected_checks: Vec<usize> = self
                    .pcm
                    .parity_check_matrix
                    .nonzero_rows(bit_index)
                    .to_vec();

                for &check_idx in &connected_checks {
                    let mut check_to_bit_msg = 0.0;

                    if self.bp_method == BpMethod::ProductSum {
                        // --- Product Sum (Sum-Product) Logic ---
                        let mut prod = 1.0;

                        // 対象のチェックノードに接続する「他の」ビットからのメッセージの積
                        // self.pcm.iterate_row(check_idx) に相当
                        let row_entries = self.pcm.iterate_row(check_idx);

                        for entry in row_entries {
                            if entry.col_index != bit_index {
                                prod *= (entry.bit_to_check_msg / 2.0).tanh();
                            }
                        }

                        let sgn_val = if syndrome[check_idx] != 0 { -1.0 } else { 1.0 };

                        // 2 * atanh(x) = ln((1+x)/(1-x))
                        let term = sgn_val * prod;
                        // 数値安定性のためのクリッピング
                        let clamped_term = term.max(-0.9999999).min(0.9999999);
                        check_to_bit_msg = ((1.0 + clamped_term) / (1.0 - clamped_term)).ln();
                    } else if self.bp_method == BpMethod::MinimumSum {
                        // --- Minimum Sum Logic ---
                        let mut min_val = f64::MAX;
                        let mut sgn = syndrome[check_idx] as i32;

                        // 対象のチェックノードに接続する「他の」ビットを探索
                        let row_entries = self.pcm.iterate_row(check_idx);

                        for entry in row_entries {
                            if entry.col_index != bit_index {
                                let abs_val = entry.bit_to_check_msg.abs();
                                if abs_val < min_val {
                                    min_val = abs_val;
                                }
                                if entry.bit_to_check_msg <= 0.0 {
                                    sgn += 1;
                                }
                            }
                        }

                        let message_sign = if sgn % 2 == 0 { 1.0 } else { -1.0 };
                        check_to_bit_msg = alpha * message_sign * min_val;
                    }

                    // エッジのメッセージを更新し、ビットのLLRに加算
                    // self.pcm.get_entry_mut(check_idx, bit_index).check_to_bit_msg = check_to_bit_msg;
                    // self.pcm.get_entry_mut(check_idx, bit_index).bit_to_check_msg = self.log_prob_ratios[bit_index]; // ここは一時的

                    // 実際にはRustではアクセサ経由で更新
                    self.pcm.update_edge_msg(check_idx, bit_index, |e| {
                        e.check_to_bit_msg = check_to_bit_msg;
                        // 注: C++コードではここで e.bit_to_check_msg = log_prob_ratios[bit_index] としているが
                        // これは「累積前のLLR」を入れている。しかしその直後に += しているので
                        // 結果的に次のStep Bで正しい「Extrinsic情報」を作るための準備となる。
                    });

                    self.log_prob_ratios[bit_index] += check_to_bit_msg;
                }

                // ---------------------------------------------------------
                // Step B: 硬判定と Bit-to-Check メッセージの更新 (Outgoing)
                // ---------------------------------------------------------

                // 硬判定
                if self.log_prob_ratios[bit_index] <= 0.0 {
                    self.decoding[bit_index] = 1;
                } else {
                    self.decoding[bit_index] = 0;
                }

                // 次のイテレーション（または次のチェックノード処理）のために
                // このビットから出るメッセージ (bit_to_check) を更新する
                // rule: M_{bit->check} = Total_LLR - M_{check->bit}
                for &check_idx in &connected_checks {
                    let total_llr = self.log_prob_ratios[bit_index];

                    self.pcm.update_edge_msg(check_idx, bit_index, |e| {
                        // Extrinsic情報の計算
                        e.bit_to_check_msg = total_llr - e.check_to_bit_msg;
                    });
                }
            }

            // 4. シンドローム計算と収束判定
            self.candidate_syndrome = self.pcm.parity_check_matrix() * &self.decoding;
            self.iterations = it;

            if self.candidate_syndrome == *syndrome {
                self.converge = true;
                return self.decoding.clone();
            }
        }

        self.decoding.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_pcm() {
        let pcm = BinarySparseMatrix::from_row_adj(
            4,
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
        );
        let decoder = BpDecoder::from_pcm(
            pcm,
            BpMethod::ProductSum,
            BpSchedule::Serial,
            10,
            0.0,
            false,
            vec![0.1; 5],
        );
        assert_eq!(decoder.bit_count, 5);
        assert_eq!(decoder.maximum_iterations, 10);
        assert_eq!(decoder.channel_probabilities.len(), 5);
    }

    #[test]
    fn test_bp_non_error() {
        let pcm = BinarySparseMatrix::from_row_adj(2, 3, vec![vec![0, 1], vec![1, 2]]);
        let mut decoder = BpDecoder {
            pcm: BpSparse::new(pcm),
            bit_count: 3,
            bp_method: BpMethod::ProductSum,
            schedule: BpSchedule::Serial,
            maximum_iterations: 10,
            ms_scaling_factor: 0.0,
            random_serial_schedule: false,
            channel_probabilities: vec![0.1; 3],
            initial_log_prob_ratios: vec![0.0; 3],
            log_prob_ratios: vec![0.0; 3],
            decoding: vec![0; 3],
            candidate_syndrome: vec![0; 2],
            converge: false,
            iterations: 0,
            serial_schedule_order: vec![0, 1, 2],
        };
        let syndrome = vec![0, 0];
        let result = decoder.decode(&syndrome);
        assert_eq!(result, vec![0, 0, 0]);
        assert!(decoder.converge);
    }

    #[test]
    fn test_bp_single_error() {
        let pcm = BinarySparseMatrix::from_row_adj(2, 3, vec![vec![0, 1], vec![1, 2]]);
        let mut decoder = BpDecoder {
            pcm: BpSparse::new(pcm),
            bit_count: 3,
            bp_method: BpMethod::ProductSum,
            schedule: BpSchedule::Serial,
            maximum_iterations: 10,
            ms_scaling_factor: 0.0,
            random_serial_schedule: false,
            channel_probabilities: vec![0.1; 3],
            initial_log_prob_ratios: vec![0.0; 3],
            log_prob_ratios: vec![0.0; 3],
            decoding: vec![0; 3],
            candidate_syndrome: vec![0; 2],
            converge: false,
            iterations: 0,
            serial_schedule_order: vec![0, 1, 2],
        };
        for i in 0..3 {
            let mut error_vector = vec![0; 3];
            error_vector[i] = 1; // 単一ビットエラー
            let syndrome = decoder.pcm.parity_check_matrix() * &error_vector;
            let result = decoder.decode(&syndrome);
            assert_eq!(result, error_vector);
            assert!(decoder.converge);
        }
    }
}
