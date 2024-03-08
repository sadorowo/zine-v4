pub fn get_type<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn no_generic_type<T>(_: &T) -> &'static str {
    let type_name = std::any::type_name::<T>();

    if let Some(index) = type_name.find('<') {
        &type_name[..index]
    } else {
        type_name
    }
}