use std::cmp;

fn get_separator(x: usize) -> &'static str {
    match x {
        2 | 5 => ".",
        8 => "/",
        12 => "-",
        _ => ""
    }
}

pub fn format(cnpj: &str) -> String {
    let cnpj = cnpj.matches(char::is_numeric).collect::<Vec<_>>();

    let mut cnpj_with_mask: String = String::from("");

    for x in 0..cmp::min(cnpj.len(), 14) {
        cnpj_with_mask.push_str(get_separator(x));
        cnpj_with_mask.push_str(cnpj[x]);
    }

    cnpj_with_mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_format_cnpj_with_mask() {
        assert_eq!(format(""), "");
        assert_eq!(format("4"), "4");
        assert_eq!(format("46"), "46");
        assert_eq!(format("468"), "46.8");
        assert_eq!(format("4684"), "46.84");
        assert_eq!(format("46843"), "46.843");
        assert_eq!(format("468434"), "46.843.4");
        assert_eq!(format("4684348"), "46.843.48");
        assert_eq!(format("46843485"), "46.843.485");
        assert_eq!(format("468434850"), "46.843.485/0");
        assert_eq!(format("4684348500"), "46.843.485/00");
        assert_eq!(format("46843485000"), "46.843.485/000");
        assert_eq!(format("468434850001"), "46.843.485/0001");
        assert_eq!(format("4684348500018"), "46.843.485/0001-8");
        assert_eq!(format("46843485000186"), "46.843.485/0001-86");
    }

    #[test]
    fn should_not_add_digits_after_the_cnpj_length() {
        assert_eq!(format("468434850001860000000000"), "46.843.485/0001-86");
    }

    #[test]
    fn should_remove_all_non_numeric_characters() {
        assert_eq!(format("46.?ABC843.485/0001-86abc"), "46.843.485/0001-86");
    }
}
