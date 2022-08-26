using Parameters

struct Token

    type::String
    s::String

end

@with_kw mutable struct Jack

    filename::String
    lines::Vector{String} = Vector{String}()

    # Tokenizer
    tokens::Vector{Token} = Vector{Token}()
    str_tokens::Vector{String} = Vector{String}()
    
    # Parser
    xml::String = ""

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

    d = Dict("<" => "&lt;", ">" => "&gt;", "&" => "&amp;")

    if word ∈ keywords
        return Token("keyword", word)
    elseif word ∈ symbols

        if haskey(d, word)
            return Token("symbol", d[word])
        else
            return Token("symbol", word)
        end

    elseif word[begin] == '"'
        return Token("stringConstant", word[begin+1:end-1])
    elseif word[begin] ∈ ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        return Token("integerConstant", word)
    else
        return Token("identifier", word)
    end

end

token_to_str(token::Token) = "<$(token.type)> $(token.s) </$(token.type)>"

