using System;
using System.Collections.Generic;
using System.Text;

using NathanWiles.Nala.Errors;
using NathanWiles.Nala.IO;
using NathanWiles.Nala.Lexing;

namespace NathanWiles.Nala.Parsing.Rules
{
    public class FunctionParseRule : ParseRule
    {
        public override bool Matches(List<NalaToken> sentence, IIOContext ioContext)
        {
            var token = sentence[0];
            if (token.value == "func")
            {
                return IsProper(sentence, ioContext);
            }

            return false;
        }

        public override bool IsProper(List<NalaToken> sentence, IIOContext ioContext)
        {
            // The second element of a function should always be an identifier token.
            if (sentence[1].type != TokenType.Identifier) { new ParseError(this, sentence[1], "Expected identifier.").Report(ioContext); return false; }

            // The third element of a function should always be a '(' character.
            if (sentence[2].value != "(") { new ParseError(this, sentence[2], "Expected '(' character.").Report(ioContext); return false; }

            int openParenPos = 2, closeParenPos = 0;

            for (int i = 2; i < sentence.Count; i++)
            {
                var token = sentence[i];

                switch (token.value)
                {
                    case ")": closeParenPos = i; break;
                }
            }

            List<NalaToken> betweenParens = sentence.GetRange(openParenPos, closeParenPos - openParenPos);

            if (!(new ParamsParseRule().Matches(betweenParens, ioContext))) return false;

            return true;
        }
    }
}
