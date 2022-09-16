@with_kw mutable struct SymbolTable

    d_class::Dict{String, Tuple{String, String, Int64}} =
        Dict{String, Tuple{String, String, Int64}}()
    d_sub::Dict{String, Tuple{String, String, Int64}} =
        Dict{String, Tuple{String, String, Int64}}()
    d_cnt::Dict{String, Int64} = Dict(
        "static" => 0,
        "field" => 0,
        "arg" => 0,
        "var" => 0
    )

end

function start_subroutine!(st::SymbolTable)
    
    st.d_sub = Dict{String, Tuple{String, String, Int64}}()
    st.d_cnt["arg"] = 0
    st.d_cnt["var"] = 0

end

function define!(st::SymbolTable, name, type, kind)

    if kind ∈ ("static", "field")
        push!(st.d_class, name => (type, kind, st.d_cnt[kind]))
    else
        push!(st.d_sub, name => (type, kind, st.d_cnt[kind]))
    end

    st.d_cnt[kind] += 1

end

