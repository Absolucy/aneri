#define ANERI_NOISE_PERLIN "perlin"
#define ANERI_NOISE_SIMPLEX "simplex"
#define ANERI_NOISE_WORLEY "worley"

/// Noise values for a whole grid, each from 0 to 1.
/// `kind` is `ANERI_NOISE_PERLIN`, `ANERI_NOISE_SIMPLEX` or `ANERI_NOISE_WORLEY`.
/// The value for cell `(x, y)`, counting from 1, is at `grid[(y - 1) * width + x]`,
/// and matches `aneri_noise_at(kind, seed, x, y, frequency, octaves)`.
/// `frequency` is how many noise features fit in one unit (default 0.01, so use
/// `1 / zoom` to zoom in). `octaves` above 1 layers that many passes of finer detail.
/// Rows are computed in parallel. Runtimes if `kind` is not a known kind.
/proc/aneri_noise_grid(kind, seed, width, height, frequency, octaves) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL(noise_grid, kind, seed, width, height, frequency, octaves)

/// Cave-like grid from cellular automata: random floor, smoothed over several passes.
/// Returns a list of 1 (floor) and 0 (wall), where cell `(x, y)`, counting from 1,
/// is at `grid[(y - 1) * width + x]`.
/// `fill_percent` is the chance (0 to 100) that a cell starts as floor.
/// Each iteration, a floor stays with at least `survival_limit` floor neighbors
/// (of 8) and a wall becomes floor with at least `birth_limit`.
/// If `edges_alive`, cells outside the grid count as floor.
/// A given `seed` always gives the same grid. Without one, it is random.
/// rust-g's `cnoise_generate` used strict comparisons, so its `birth_limit`
/// needs +1 here, and its `death_limit` is this `survival_limit`.
/proc/aneri_noise_cellular_automata(width, height, fill_percent, iterations, birth_limit, survival_limit, edges_alive = FALSE, seed) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL(noise_cellular_automata, width, height, fill_percent, iterations, birth_limit, survival_limit, edges_alive, seed)

/// Evenly spread random points ("blue noise"), no two closer than `radius`.
/// Returns a list of `list(x, y)`, with x from 1 to `width` and y from 1 to `height`.
/// A `radius` under 1 is treated as 1. A given `seed` always gives the same points.
/proc/aneri_noise_poisson(width, height, radius, seed) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL(noise_poisson, width, height, radius, seed)

/// Cave level with rooms joined by corridors, smoothed into natural caves.
/// Returns a list of 1 (floor) and 0 (wall), where cell `(x, y)`, counting from 1,
/// is at `grid[(y - 1) * width + x]`. Every floor is connected to a room,
/// a corridor, or an open prefab.
///
/// The map is split into rooms, neighboring rooms are joined by corridors, the
/// rest is filled with random floor, and cellular automata smooth it all.
/// Rooms, corridors and prefabs are never changed by the smoothing.
///
/// `settings` is an assoc list, every key optional (default in brackets):
/// * `min_bsp_size`: smallest side of a room's area [20]
/// * `max_ratio`: how stretched an area may get before it is split the other way [2.5]
/// * `padding`: gap between a room and its area's edge [2]
/// * `room_fill_percent`: how much of an area a room fills, 0 to 100 [80]
/// * `corridor_width` [1]
/// * `loop_percent`: chance of extra corridors that make loops, 0 to 100 [15]
/// * `noise_percent`: chance a leftover cell starts as floor, 0 to 100 [48]
/// * `ca_steps`: smoothing passes [6]
/// * `birth_limit` and `survival_limit`: as in `aneri_noise_cellular_automata` [5, 4]
/// * `edges_alive`: whether cells outside the map count as floor, which opens tunnels at the map edge [FALSE]
/// An unknown key runtimes, so typos don't go unnoticed.
///
/// `prefabs` is a list of `list("x" = , "y" = , "width" = , "height" = , "enclosed" = )`,
/// where x and y are the bottom-left turf, counting from 1. An enclosed prefab
/// is kept solid wall, an open one is kept as floor.
/// A given `seed` always gives the same map. Without one, it is random.
/proc/aneri_noise_cave_system(width, height, list/settings, list/prefabs, seed) as /list
	RETURN_TYPE(/list)
	return ANERI_CALL(noise_cave_system, width, height, settings, prefabs, seed)
