pub fn map<MapperFunction, InputType, OutputType>(
    input: Vec<InputType>,
    mut mapper: MapperFunction,
) -> Vec<OutputType>
where
    MapperFunction: FnMut(InputType) -> OutputType,
{
    let mut result = Vec::new();
    for element in input.into_iter() {
        result.push(mapper(element));
    }
    result
}
