include("tokenize.jl")
include("symbol_table.jl")

@with_kw mutable struct Parser

    tokens::Vector{Token}
    # Current Token
    idx::Int64 = 1
    token::Token = tokens[idx]
    token_previous::Token = tokens[idx]
    has_return::Bool = false

    # Symbol Table
    st::SymbolTable = SymbolTable()
    name_class::String = ""
    name_func::String = ""
    type_subroutine::String = ""
    n_exp::Int64 = 0
    n_if::Int64 = 0
    n_while::Int64 = 0
    d_kind::Dict{String, String} = Dict(
        "arg" => "argument",
        "static" => "static",
        "var" => "local",
        "field" => "this"
    )

    # VM Code
    codes::Vector{String} = Vector{String}()

end

function advance!(p::Parser)

    p.token_previous = p.token
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
        $(compile_terminal(p, "class"))"""
        
        p.name_class = p.token.s

        xml =
        """
        $xml
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

        kind = p.token.s
        
        xml =
        """
        <classVarDec>
        $(compile_terminal(p))"""

        type = p.token.s

        xml =
        """
        $xml
        $(compile_terminal(p))"""

        define!(p.st, p.token.s, type, kind)

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_var_identifiers(p, type, kind))
        $(compile_terminal(p, ";"))
        </classVarDec>
        $(compile_class_var_dec(p))"""

        return xml
    else
        return ""
    end

end


function compile_var_identifiers(p::Parser, type, kind)

    if p.token.s == ","
        
        xml =
        """
        $(compile_terminal(p, ","))"""

        define!(p.st,p.token.s, type, kind)

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_var_identifiers(p, type, kind))"""

        return xml
    else
        return ""
    end

end

function compile_subroutine(p::Parser)

    if p.token.s ∈ ["constructor", "function", "method"]

        p.type_subroutine = p.token.s

        start_subroutine!(p.st)

        xml =
        """
        <subroutineDec>
        $(compile_terminal(p))
        $(compile_terminal(p))"""

        p.name_func = p.token.s

        xml =
        """
        $xml
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


    num = 0
    while p.token.s == "var"
        
        xml =
        """
        $xml
        $(compile_var_dec(p))"""

        num += 1

    end

    push!(p.codes, "function $(p.name_class).$(p.name_func) $num")

    if p.type_subroutine == "constructor"

        push!(p.codes, "push const $(p.st.d_cnt["field"])")
        push!(p.codes, "call Memory.alloc 1")
        push!(p.codes, "pop pointer 0")

    elseif p.type_subroutine == "method"

        push!(p.codes, "push arg 0")
        push!(p.codes, "pop pointer 0")

    end


    p.has_return = false
    xml =
    """
    $xml
    $(compile_statements(p))
    $(compile_terminal(p, "}"))
    </subroutineBody>"""

    if !p.has_return

        push!(p.codes, "push constant 0")
        push!(p.codes, "return")

    end


    return xml

end

function compile_subroutine_call(p::Parser)

    name = p.token_previous.s
    xml = ""
    p.n_exp = 0
    name_vm = ""
    if p.token.s == "."

        xml =
        """
        $xml
        $(compile_terminal(p, "."))"""

        if haskey(p.st.d_sub, name)
            type, kind, num = p.st.d_sub[name]
            kind = p.d_kind[kind]
            push!(p.codes, "push $kind $num")
            p.n_exp += 1
            name_vm = "$(p.st.d_sub[name][1]).$(p.token.s)"
        else
            name_vm = "$name.$(p.token.s)"
        end

        xml =
        """
        $xml
        $(compile_terminal(p))"""

    else
        
        name_vm = "$name.$(p.token.s)"

    end

    xml =
    """
    $xml
    $(compile_terminal(p, "("))
    $(compile_expression_list(p))
    $(compile_terminal(p, ")"))"""

    push!(p.codes, "call $name_vm $(p.n_exp)")


    return xml
end

function compile_parameter_list(p::Parser)

    if p.token.s == ")"
        return "<parameterList>\n</parameterList>"
    else

        kind = "arg"
        type = p.token.s

        xml =
        """
        <parameterList>
        $(compile_terminal(p))"""

        define!(p.st, p.token.s, type, kind)

        xml =
        """
        $xml
        $(compile_terminal(p))"""

        while p.token.s == ","

            xml =
            """
            $xml
            $(compile_terminal(p, ","))"""
            
            type = p.token.s

            xml =
            """
            $xml
            $(compile_terminal(p))"""
            
            define!(p.st,p.token.s, type, kind)

            xml =
            """
            $xml
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
    $(compile_terminal(p, "var"))"""

    kind = "var"
    type = p.token.s

    xml =
    """
    $xml
    $(compile_terminal(p))"""

    define!(p.st,p.token.s, type, kind)

    xml =
    """
    $xml
    $(compile_terminal(p))"""

    while p.token.s == ","

        xml =
        """
        $xml
        $(compile_terminal(p, ","))"""
        
        define!(p.st,p.token.s, type, kind)

        xml =
        """
        $xml
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

        if p.token.s == "return"
            p.has_return = true
        end

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

    push!(p.codes, "pop temp 0")

    return xml
end

function compile_let(p::Parser)

    xml =
    """
    <letStatement>
    $(compile_terminal(p, "let"))"""
    
    name_var = p.token.s
    type, kind, num = p.st.d_sub[name_var]
    kind = p.d_kind[kind]

    xml =
    """
    $xml
    $(compile_terminal(p))"""

    if p.token.s != "="

        xml =
        """
        $xml
        $(compile_terminal(p, "["))
        $(compile_expression(p))
        $(compile_terminal(p, "]"))"""

        push!(p.codes, [
            "push $kind $num",
            "add",
            "pop temp 0"
        ])

        xml =
        """
        $xml
        $(compile_terminal(p, "="))
        $(compile_expression(p))"""

        push!(p.codes, [
            "push temp 0",
            "pop pointer 1",
            "pop that 0"
        ])
    
    else

        xml =
        """
        $xml
        $(compile_terminal(p, "="))
        $(compile_expression(p))"""

        push!(p.codes, "pop $kind $num")

    end

    xml =
    """
    $xml
    $(compile_terminal(p, ";"))
    </letStatement>"""

    return xml
end

function compile_while(p::Parser)

    l1 = "WHILE_EXP$(p.n_while)"
    l2 = "WHILE_END$(p.n_while)"
    p.n_while += 1

    push!(p.codes, "label $l1")

    xml =
    """
    <whileStatement>
    $(compile_terminal(p, "while"))
    $(compile_terminal(p, "("))
    $(compile_expression(p))
    $(compile_terminal(p, ")"))
    $(compile_terminal(p, "{"))"""
    
    push!(p.codes, "if-goto $l2")
    xml =
    """
    $xml
    $(compile_statements(p))"""
    
    push!(p.codes, "goto $l1")
    push!(p.codes, "label $l2")

    xml =
    """
    $xml
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
        push!(p.codes, "push constant 0")
        push!(p.codes, "return")
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
    $(compile_terminal(p, ")"))"""
    
    
    l1 = "IF_TRUE$(p.n_if)"
    l2 = "IF_FALSE$(p.n_if)"
    l3 = "IF_END$(p.n_if)"

    push!(p.codes, "if-goto $l1")
    push!(p.codes, "goto $l2")
    push!(p.codes, "label $l1")
    p.n_if += 1

    xml = 
    """
    $xml
    $(compile_terminal(p, "{"))
    $(compile_statements(p))"""
    
    push!(p.codes, "goto $l3")
    
    xml =
    """
    $xml
    $(compile_terminal(p, "}"))"""

    push!(p.codes, "label $l2")

    if p.token.s == "else"
        xml =
        """
        $xml
        $(compile_terminal(p, "else"))
        $(compile_terminal(p, "{"))
        $(compile_statements(p))
        $(compile_terminal(p, "}"))"""
    end

    push!(p.codes, "label $l3")

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

    d_op = Dict(
        "&" => "and",
        "*" => "call Math.multiply 2",
        "+" => "add",
        "-" => "sub",
        "/" => "call Math.divide 2",
        "|" => "or",
        "&lt;" => "lt",
        "=" => "eq",
        "&gt;" => "gt",
        "&amp;" => "and")


    while p.token.s ∈ keys(d_op)

        op = p.token.s

        xml =
        """
        $xml
        $(compile_terminal(p))
        $(compile_term(p))"""

        push!(p.codes, d_op[op])

    end

    xml =
    """
    $xml
    </expression>"""

end

function compile_term(p::Parser)

    xml = "<term>"

    if p.token.type ∈ ["keyword", "integerConstant", "stringConstant"]

        if p.token.type == "integerConstant"
            push!(p.codes, "push constant $(p.token.s)")
        end

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

        if p.token.s == "~"
            push!(p.codes, "not")
        else
            push!(p.codes, "neg")
        end

    elseif p.token.s == "("

        xml =
        """
        $xml
        $(compile_terminal(p, "("))
        $(compile_expression(p))
        $(compile_terminal(p, ")"))"""
    
    else

        name_var = p.token.s

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

            type, kind, num = p.st.d_sub[name_var]
            kind = p.d_kind[kind]
            push!(p.codes, [
                "push $kind $num",
                "add",
                "pop pointer 1",
                "push that 0"
            ])
        
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

        p.n_exp += 1

        while p.token.s == ","

            xml =
            """
            $xml
            $(compile_terminal(p))
            $(compile_expression(p))"""

            p.n_exp += 1

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
    jack.code = join(p.codes, "\n")
    
end

