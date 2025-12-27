use crate::decoder::traits::Decoder;

enum BpMethod {
    SumProduct,
    MinSum,
}

enum BpSchedule {
    Parallel,
    Serial,
}

pub struct BpDecoderBinary {
    max_iterations: usize,
    error_rate: f64,
    bp_method: BpMethod,
    ms_scaling_factor: f64,
    schedule: BpSchedule,
}

impl BpDecoderBinary {
    pub fn new(
        max_iterations: usize,
        error_rate: f64,
        bp_method: &str,
        ms_scaling_factor: f64,
        schedule: &str,
    ) -> Self {
        let bp_method = match bp_method {
            "SumProduct" => BpMethod::SumProduct,
            "MinSum" => BpMethod::MinSum,
            _ => panic!("存在しないBP methodです: {}", bp_method),
        };
        let schedule = match schedule {
            "Parallel" => BpSchedule::Parallel,
            "Serial" => BpSchedule::Serial,
            _ => panic!("存在しないBP scheduleです: {}", schedule),
        };
        Self {
            max_iterations,
            error_rate,
            bp_method,
            ms_scaling_factor,
            schedule,
        }
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn error_rate(&self) -> f64 {
        self.error_rate
    }

    pub fn bp_method(&self) -> String {
        match self.bp_method {
            BpMethod::SumProduct => "SumProduct".to_string(),
            BpMethod::MinSum => "MinSum".to_string(),
        }
    }

    pub fn ms_scaling_factor(&self) -> f64 {
        self.ms_scaling_factor
    }

    pub fn schedule(&self) -> String {
        match self.schedule {
            BpSchedule::Parallel => "Parallel".to_string(),
            BpSchedule::Serial => "Serial".to_string(),
        }
    }
}

impl Decoder for BpDecoderBinary {
    fn name(&self) -> &str {
        "BP Decoder (Binary)"
    }

    fn decode(
        &self,
        _syndrome: &crate::code::error_vector::Syndrome,
    ) -> crate::code::error_vector::ErrorVector {
        todo!("BPデコーダの実装を行う")
    }
}
