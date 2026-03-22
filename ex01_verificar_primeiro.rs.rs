pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    lista.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste() {
        assert_eq!(verificar_primeiro(&[1,2,3]), Some(1));
        assert_eq!(verificar_primeiro(&[]), None);
    }
}