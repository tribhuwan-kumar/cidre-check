use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(pub AdamOptimizer(mlc::Optimizer));
impl AdamOptimizer {
    define_cls!(MLC_ADAM_OPTIMIZER);

    #[objc::msg_send(beta1)]
    pub fn beta1(&self) -> f32;

    #[objc::msg_send(beta2)]
    pub fn beta2(&self) -> f32;

    #[objc::msg_send(epsilon)]
    pub fn epsilon(&self) -> f32;

    #[objc::msg_send(usesAMSGrad)]
    pub fn uses_ams_grad(&self) -> bool;

    #[objc::msg_send(timeStep)]
    pub fn time_step(&self) -> usize;

    #[objc::msg_send(optimizerWithDescriptor:)]
    pub fn with_desc(desc: &mlc::OptimizerDesc) -> arc::R<Self>;

    #[objc::msg_send(optimizerWithDescriptor:beta1:beta2:epsilon:timeStep:)]
    pub fn with_desc_betas_epsilon_time_step(
        desc: &mlc::OptimizerDesc,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        time_step: usize,
    ) -> arc::R<Self>;

    #[objc::msg_send(optimizerWithDescriptor:beta1:beta2:epsilon:usesAMSGrad:timeStep:)]
    pub fn with_desc_betas_epsilon_uses_ams_grad_time_step(
        desc: &mlc::OptimizerDesc,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        uses_ams_grad: bool,
        time_step: usize,
    ) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
unsafe extern "C" {
    static MLC_ADAM_OPTIMIZER: &'static objc::Class<AdamOptimizer>;
}
