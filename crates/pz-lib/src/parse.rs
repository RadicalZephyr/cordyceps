use nom::{
    bytes::complete::is_not,
    character::complete::{alpha1, char, multispace0},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn item_attr<'a>(input: &'a str) -> IResult<&'a str, (&'a str, &'a str)> {
    separated_pair(
        alpha1,
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
    }
}
