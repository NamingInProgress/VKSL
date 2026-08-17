#[derive(Clone, Debug)]
pub enum UniformModifier {
    Readonly,
    PackingType(PackingType)
}

#[derive(Clone, Debug)]
pub enum PackingType {
    STD140,
    STD430
}