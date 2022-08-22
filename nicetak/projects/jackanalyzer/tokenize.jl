using Parameters

struct Token

    type::String
    s::String

end

@with_kw struct Jack

    filename::String
    lines::Vector{String} = Vector{String}()
    tokens::Vector{Token} = Vector{Token}()
    str_tokens::Vector{String} = Vector{String}()
    
    # Parser
    idx_current::Int64 = 1
    token_current::Token = nothing
    str_compileds::Vector{String} = Vector{String}()

end


function tokenize!(jack::Jack)


    push!(jack.str_tokens, "<tokens>")

    for line ∈ jack.lines

        words = line_to_words(line)
        
        for word ∈ words
            token = tokenize(word)
            push!(jack.tokens, token)
            push!(jack.str_tokens, token_to_str(token))
        end
    
    end

    push!(jack.str_tokens, "</tokens>\n")

end

symbols = ["{", "}", "(", ")", "[", "]", ".", ",", ";", "+", "-", "*", "/", "&", "|", "<", ">", "=", "~"]
keywords = ["class", "method", "function", "constructor", "int", "boolean", "char", "void", "var", "static", "field", "let", "do", "if", "else", "while", "return", "true", "false", "null", "this"]

function line_to_words(line, symbols = symbols)

    for s ∈ symbols
        line = replace(line, "$s" => " $s ")
    end

    i₋₁ = 1
    in_string = false
    words = Vector{String}()

    for i = 1:length(line)

        if line[i] == '"' && !in_string
            
            append!(words, split(line[i₋₁:i-1]))
            in_string = true
            i₋₁ = i

        elseif line[i] == '"' && in_string

            push!(words, line[i₋₁:i])
            in_string = false
            i₋₁ = i + 1

        elseif i == length(line)

            append!(words, split(line[i₋₁:i]))

        end

    end

    return words
end

function tokenize(word, symbols = symbols, keywords = keywords)

    if word ∈ keywords
        return Token("keyword", word)
    elseif word ∈ symbols
        return Token("symbol", word)
    elseif word[begin] == '"'
        return Token("stringConstant", word[begin+1:end-1])
    elseif word[begin] ∈ ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        return Token("integerConstant", word)
    else
        return Token("identifier", word)
    end

end

function token_to_str(token::Token)

    d = Dict("<" => "&lt;", ">" => "&gt;", "&" => "&amp;")

    if token.type == "symbol" && haskey(d, token.s)
        return "<$(token.type)> $(d[token.s]) </$(token.type)>"
    else
        return "<$(token.type)> $(token.s) </$(token.type)>"
    end

end
