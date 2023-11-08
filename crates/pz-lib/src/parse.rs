use nom::{
    bytes::streaming::is_not,
    character::streaming::{alpha1, char, multispace0},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

fn item_attr<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    separated_pair(
        preceded(multispace0, alpha1),
        delimited(multispace0, char('='), multispace0),
        terminated(is_not(","), char(',')),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_attr_test() {
        assert_eq!(
            Ok(("", ("DisplayCategory", "Camping"))),
            item_attr("DisplayCategory = Camping,")
        );
        assert_eq!(
            Ok(("", ("DisplayName", "Campfire Materials"))),
            item_attr("DisplayName                     =        Campfire Materials,")
        );
        assert_eq!(
            Ok(("", ("Weight", "0.4"))),
            item_attr("                Weight                          =               0.4,")
        )
    }
}
