// grammer of ebnf
// ```ebnf
// <ebnf> ::= { <rule> ";" } ;
// -> fn parse_[$rule.character.name](&mut self) -> Return<[$rule.character.upper_name]> {
//        [$rule.ret]
//    }
// 
// <rule> ::= <character> "::=" <alternative> ;
// -> [$alternative.ret]
// 
// <alternative> ::= <enum> | <struct> ;
// -> $enum.ret | $struct.ret
// 
// 
// <enum> ::= <enum_item> { "|" <enum_item> } ;
// -> if let Ok([$enum_item.0]) = self.parse_[$enum.item.0.name]() {
//        Ok([$enum.upper_name]::[$enum_item.terminal]([$enum_item.0.ret]))
//    } else if let Ok([$enum_item.$i]) = self.parse_[$enum.item.$i.name]() {
//        Ok([$enum.upper_name]::[$enum_item.terminal]([$enum_item.$1.ret]))
//    } else {
//        Err(ParseErr)
//    }
// 
// <enum_item> ::= { <terminal> } <character> { <terminal> } "as" <terminal> | { <terminal> } "as" <terminal> ;
// -> self.next(Token::[$token_name_map(terminal.$i.name)])?;
//    let [$character.name] = self.parse_[$character.name]()?;
//    self.next(Token::[$token_name_map(terminal.$i.name)])?;
//    |
//    self.next(Token::[$token_name_map(terminal.$i.name)])?;
// 
// 
// <struct> ::= <struct_item> { <struct_item> } ;
// -> self.parse_[$struct.item.$i.name]()?;
// 
// <struct_item> ::= "{" <character> "}" | "[" <character> "]" | <character> ;
// -> let [$character.name] = self.ctx.arena.iter_with(Self::parse_[$character.name]);
//    |
//    let [$character.name] = self.parse_[$character.name]().into_option();
//    |
//    let [$character.name] = self.parse_[$character.name]()?;
// 
// <terminal> ::= "\"" \.*\ "\"" 
// <character> ::= "<" \[a-zA-Z_]*\ ">"
// ```