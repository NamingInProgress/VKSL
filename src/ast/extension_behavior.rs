#[derive(Clone, Debug)]
pub enum ExtensionBehavior {
    Enable,
    Require,
    Warn,
    Disable
}