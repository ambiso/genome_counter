### A Pluto.jl notebook ###
# v0.12.21

using Markdown
using InteractiveUtils

# ╔═╡ 8a73f21c-806c-11eb-2dc9-3bebf2b7096e
using LinearAlgebra

# ╔═╡ f705f48e-806c-11eb-0a3a-6bfbd37ede4a
vars = collect("acgt")

# ╔═╡ a5766f68-806c-11eb-3bd3-11c9478de56f
ints = convert.(Int, uppercase.(vars))

# ╔═╡ 997d55b4-806c-11eb-1f2d-572b97b08299
M = convert.(Rational{Int}, hcat([ints .& 7, (ints .>> 1) .& 7, (ints .>> 2) .& 7, ones(4)]...)')

# ╔═╡ 9babd860-806c-11eb-1d6a-e394fde7f079
@assert rank(M) == 4

# ╔═╡ a79895fc-806c-11eb-130f-69a1bc30d65b
mi = inv(M)

# ╔═╡ b51c04da-806c-11eb-1d38-e341f90bd8e3
function generate_code(fname, m)
	open(fname, "w") do f
		for (i, row) in enumerate(eachrow(m))
			print(f, "let $(vars[i])=")
			for (j, col) in enumerate(row)
				print(f, "Ratio::new(sum$(j), 1) * ")
				print(f, "Ratio::new($(numerator(col)), $(denominator(col)))")
				if j != length(row)
					print(f, "+")
				else
					println(f, ";")
				end
			end
		end
	end
end

# ╔═╡ 3a17f222-806d-11eb-12f8-fdbad76d21e8
generate_code("code_gen.rs", mi)

# ╔═╡ Cell order:
# ╠═8a73f21c-806c-11eb-2dc9-3bebf2b7096e
# ╠═f705f48e-806c-11eb-0a3a-6bfbd37ede4a
# ╠═a5766f68-806c-11eb-3bd3-11c9478de56f
# ╠═997d55b4-806c-11eb-1f2d-572b97b08299
# ╠═9babd860-806c-11eb-1d6a-e394fde7f079
# ╠═a79895fc-806c-11eb-130f-69a1bc30d65b
# ╠═b51c04da-806c-11eb-1d38-e341f90bd8e3
# ╠═3a17f222-806d-11eb-12f8-fdbad76d21e8
