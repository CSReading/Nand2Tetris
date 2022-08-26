include("tokenize.jl")

@with_kw mutable struct Parser

    tokens::Vector{Token}
    # Current Token
    idx::Int64 = 1
    token::Token = tokens[idx]

end

function advance!(p::Parser)

    p.idx += 1
    if p.idx ≤ length(p.tokens)
        p.token = p.tokens[p.idx]
    end

end


function compile_terminal(p::Parser, s)

    if p.token.type ∈ ["symbol", "keyword", "integerConstant", "stringConstant", "identifier"]
        
        token = p.token
        advance!(p)

        if s == token.s
            return "<$(token.type)> $(token.s) </$(token.type)>"
        else
            throw(ErrorException("Expected \"$s\" but, got \"$(token.s)\""))
        end

    end

end

function compile_terminal(p::Parser)

    if p.token.type ∈ ["symbol", "keyword", "integerConstant", "stringConstant", "identifier"]
        
        token = p.token
        advance!(p)
        return "<$(token.type)> $(token.s) </$(token.type)>"
    
    end

end

function compile_class(p::Parser)

    if p.token.s == "class"

        xml =
        """
        <class>
        $(compile_terminal(p, "class"))
        $(compile_terminal(p))
        $(compile_terminal(p, "{"))
        $(compile_class_var_dec(p))
        $(compile_subroutine(p))
        $(compile_terminal(p, "}"))
        </class>
        """

        return xml
    end

end

function compile_class_var_dec(p::Parser)

    if p.token.s ∈ ["static", "field"]

        xml =
        """
        <classVarDec>
        $(compile_terminal(p))
        $(compile_terminal(p))
        $(compile_terminal(p))
        $(compile_var_identifiers(p))
        $(compile_terminal(p, ";"))
        </classVarDec>
        $(compile_class_var_dec(p))"""

        return xml
    else
        return ""
    end

end


function compile_var_identifiers(p::Parser)

    if p.token.s == ","
        
        xml =
        """
        $(compile_terminal(p, ","))
        $(compile_terminal(p))
        $(compile_var_identifiers(p))"""

        return xml
    else
        return ""
    end

end

function compile_subroutine(p::Parser)

    if p.token.s ∈ ["constructor", "function", "method"]

        xml =
        """
        <subroutineDec>
        $(compile_terminal(p))
        $(compile_terminal(p))
        $(compile_terminal(p))
        $(compile_terminal(p, "("))
        $(compile_parameter_list(p))
        $(compile_terminal(p, ")"))
        $(compile_subroutine_body(p))
        </subroutineDec>
        $(compile_subroutine(p))"""

        return xml

    else
        return ""
    end

end

function compile_subroutine_body(p::Parser)

    xml =
    """
    <subroutineBody>
    $(compile_terminal(p, "{"))"""

    while p.token.s == "var"
        
        xml =
        """
        $xml
        $(compile_var_dec(p))"""

    end

    xml =
    """
    $xml
    $(compile_statements(p))
    $(compile_terminal(p, "}"))
    </subroutineBody>"""

    return xml

end

function compile_subroutine_call(p::Parser)

    xml = ""

    if p.token.s == "."

        xml =
        """
        $xml
        $(compile_terminal(p, "."))
        $(compile_terminal(p))"""

    end

    xml =
    """
    $xml
    $(compile_terminal(p, "("))
    $(compile_expression_list(p))
    $(compile_terminal(p, ")"))"""

    return xml
end



function compile_parameter_list(p::Parser)

    if p.token.s == ")"
        return "<parameterList>\n</parameterList>"
    else
        xml =
        """
        <parameterList>
        $(compile_terminal(p))
        $(compile_terminal(p))"""

        while p.token.s == ","

            xml =
            """
            $xml
            $(compile_terminal(p, ","))
            $(compile_terminal(p))
            $(compile_terminal(p))"""

        end

        xml =
        """
        $xml
        </parameterList>"""

        return xml
    end

end

function compile_var_dec(p::Parser)

    xml =
    """
    <varDec>
    $(compile_terminal(p, "var"))
    $(compile_terminal(p))
    $(compile_terminal(p))"""

    while p.token.s == ","

        xml =
        """
        $xml
        $(compile_terminal(p, ","))
        $(compile_terminal(p))"""

    end

    xml =
    """
    $xml
    $(compile_terminal(p, ";"))
    </varDec>"""

    return xml
end

function compile_statements(p::Parser)

    d = Dict(
        "while" => compile_while,
        "if" => compile_if,
        "return" => compile_return,
        "let" => compile_let,
        "do" => compile_do
    )

    xml =
    """
    <statements>"""
    
    while p.token.s ∈ keys(d)

        xml =
        """
        $xml
        $(d[p.token.s](p))"""

    end

    xml =
    """
    $xml
    </statements>"""

    return xml
end


function compile_do(p::Parser)

    xml =
    """
    <doStatement>
    $(compile_terminal(p, "do"))
    $(compile_terminal(p))
    $(compile_subroutine_call(p))
    $(compile_terminal(p, ";"))
    </doStatement>"""

    return xml
end

function compile_let(p::Parser)

    xml =
    """
    <letStatement>
    $(compile_terminal(p, "let"))
    $(compile_terminal(p))"""

    if p.token.s != "="

        xml =
        """
        $xml
        $(compile_terminal(p, "["))
        $(compile_expression(p))
        $(compile_terminal(p, "]"))"""

    end

    xml =
    """
    $xml
    $(compile_terminal(p, "="))
    $(compile_expression(p))
    $(compile_terminal(p, ";"))
    </letStatement>"""

    return xml
end

function compile_while(p::Parser)

    xml =
    """
    <whileStatement>
    $(compile_terminal(p, "while"))
    $(compile_terminal(p, "("))
    $(compile_expression(p))
    $(compile_terminal(p, ")"))
    $(compile_terminal(p, "{"))
    $(compile_statements(p))
    $(compile_terminal(p, "}"))
    </whileStatement>"""

    return xml
end

function compile_return(p::Parser)

    xml =
    """
    <returnStatement>
    $(compile_terminal(p, "return"))"""

    if p.token.s == ";"
        # Skip
    else
        xml =
        """
        $xml
        $(compile_expression(p))"""
    end

    xml =
    """
    $xml
    $(compile_terminal(p, ";"))
    </returnStatement>"""

    return xml

end

function compile_if(p::Parser)

    xml =
    """
    <ifStatement>
    $(compile_terminal(p, "if"))
    $(compile_terminal(p, "("))
    $(compile_expression(p))
    $(compile_terminal(p, ")"))
    $(compile_terminal(p, "{"))
    $(compile_statements(p))
    $(compile_terminal(p, "}"))"""

    if p.token.s == "else"
        xml =
        """
        $xml
        $(compile_terminal(p, "else"))
        $(compile_terminal(p, "{"))
        $(compile_statements(p))
        $(compile_terminal(p, "}"))"""
    end

    xml =
    """
    $xml
    </ifStatement>"""

    return xml
end


function compile_expression(p::Parser)

    xml =
    """
    <expression>
    $(compile_term(p))"""

    while p.token.s ∈ ["&", "*", "+", "-", "/", "|", "&lt;", "=", "&gt;", "&amp;"]

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_term(p))"""

    end

    xml =
    """
    $xml
    </expression>"""

end

function compile_term(p::Parser)

    xml = "<term>"

    if p.token.type ∈ ["keyword", "integerConstant", "stringConstant"]

        xml =
        """
        $xml
        $(compile_terminal(p))"""
    
    elseif p.token.s ∈ ["~", "-"]

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_term(p))"""

    elseif p.token.s == "("

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_expression(p))
        $(compile_terminal(p, ")"))"""
    
    else

        xml =
        """
        $xml
        $(compile_terminal(p))"""

        if p.token.s == "["

            xml =
            """
            $xml
            $(compile_terminal(p, "["))
            $(compile_expression(p))
            $(compile_terminal(p, "]"))"""
        
        elseif p.token.s ∈ [".", "("]

            xml =
            """
            $xml
            $(compile_subroutine_call(p))"""

        end
    end


    xml =
    """
    $xml
    </term>"""

    return xml
end


function compile_expression_list(p::Parser)

    if p.token.s == ")"
        
        return "<expressionList>\n</expressionList>"

    else

        xml =
        """
        <expressionList>
        $(compile_expression(p))"""

        while p.token.s == ","

            xml =
            """
            $xml
            $(compile_terminal(p))
            $(compile_expression(p))"""

        end

        xml =
        """
        $xml
        </expressionList>"""

        return xml

    end
    
end

function parse!(jack::Jack)

    p = Parser(tokens = jack.tokens)
    xml = compile_class(p)
    jack.xml = replace(xml, "\n\n" => "\n")

end

