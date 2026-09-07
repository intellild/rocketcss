use super::*;

type ContainerPrelude<'i> = (
    Option<AstStr<'i>>,
    Option<NodeId<'i, rocketcss_ast::ContainerCondition<'i>>>,
);

pub(in crate::parser) fn parse_container_prelude<'i>(
    input: &mut Compiler<'i>,
    prelude: &'i str,
) -> Result<ContainerPrelude<'i>, ParseError<'i, ParserError<'i>>> {
    let allocator = input.allocator();
    input.with_source(prelude, |input| {
        let name = input.try_parse(Compiler::expect_ident).ok();
        input.skip_whitespace();
        let condition = if input.is_exhausted() {
            None
        } else {
            let tokens = collect_tokens(input, allocator, 0)?;
            Some(store_node(
                rocketcss_ast::ContainerCondition::Unknown(store_vec(tokens, input)),
                input,
            ))
        };
        if name.is_none() && condition.is_none() {
            return Err(input.new_custom_error(ParserError::InvalidValue));
        }
        Ok((name.map(|name| input.add_str(name)), condition))
    })
}
