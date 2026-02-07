# Usabillity Review

Currently this library is very hard to use.

## [BEFORE] Overview

### [BEFORE] Basic usage:

Algorithm generation:

```rust
let mut grid = Grid::new(80, 60);
algorithms::get("bsp").unwrap().generate(&mut grid, 12345);
```

_How do we set parameters?_

Algorithm parameters:

```rust
let config = BspConfig {
    min_room_size: 6,
    max_room_size: 15,
    min_depth: 3,
    max_depth: 8,
    room_ratio: 0.45,
};

let mut grid = Grid::new(80, 60);
let bsp = Bsp::new(config);
bsp.generate(&mut grid, 12345);
```

_*How do we pipe multiple algorithms together?*_

Pipeline generation:

```rust
let mut pipeline = Pipeline::new();
pipeline.add_operation(ConditionalOperation::simple(PipelineOperation::Algorithm {
        name: "cellular".to_string(),
        seed: Some(12345),
    })); //Why does it have to be wrapped in ConditionalOperation?
pipeline.add_operation(ConditionalOperation::conditional(
        PipelineOperation::Log {
            message: "Checking density".to_string(),
        },
        PipelineCondition::Density {//condition
            min: Some(0.3),
            max: Some(0.7),
        },
        vec![ //if_true
            ConditionalOperation::simple(PipelineOperation::SetParameter { //Oh god...
                key: "quality".to_string(),
                value: "good".to_string(),
            }),
            ConditionalOperation::simple(PipelineOperation::Log {
                message: "Density is acceptable".to_string(),
            }),
        ],
        vec![...]//if_false
));
let mut grid = Grid::new(40, 30);
let mut context = PipelineContext::new();
let mut rng = Rng::new(12345);

let result = pipeline.execute(&mut grid, &mut context, &mut rng);
```

_*How do we set parameters for each algorithm in the pipeline?*_
_*Why is this so complicated?*_

Layered generation:

```rust
how is this even done?
```

### Questions

What is? `terrain_forge::{compose::Pipeline}` And what connection does it have to `terrain_forge::{pipeline::*}`?
What is? `errain_forge::{pipeline::TemplateLibrary}` And how to use it?

### Bug/Issue Summary

Bug/issue notes:

1. nose-fill threshold seems to be inverted.
2. Perlin noise is repeating (probably not implemented correctly)
3. Noise lacking important options: Size/scale, range (standard should be [0,1]), (can't create fractal-noise without scale)
4. What noise-fill option "frequency" even do?
5. noise-fill should consider if we want between two values instead of threshold, fill-range=[0.3,0.7]

## [Ideal] Overview

### [Ideal] Basic usage:

Algorithm generation:

```rust
let config = BspConfig { //Optional
    min_room_size: 6,
    max_room_size: 15,
    min_depth: 3,
    max_depth: 8,
    room_ratio: 0.45,
};
let mut grid = Grid::new(80, 60);
algorithms::get("bsp",config).unwrap().generate(&mut grid, 12345);//Config optional
```

Or

```rust
//Declare config as just a list of key-value pairs
let config = HashMap::from([
    ("min_room_size", "6"),
    ("max_room_size", "15"),
    ("min_depth", "3"),
    ("max_depth", "8"),
    ("room_ratio", "0.45"),
]);
let mut grid = Grid::new(80, 60);
algorithms::get("bsp").unwrap().generate(&mut grid, 12345, config);//Config optional
```

Or

```rust
let config = HashMap::from([
    ("min_room_size", "6"),
    ("max_room_size", "15"),
    ("min_depth", "3"),
    ("max_depth", "8"),
    ("room_ratio", "0.45"),
]);
let mut grid = Grid::new(80, 60);
algorithms::exec("bsp",&mut grid, 12345, config);//Config optional
```

Pipeline generation:

```rust
let connectivity_requirement = 0.7;
let spawn_point: (10,10);
let mut rng = Rng::new(seed);

let cellular_config = HashMap::from([
    ("initial_floor_chance", 0.45),
    ("iterations", 4),
    ("birth_limit", 5),
    ("death_limit", 4),
]);
let gsb_config = HashMap::from([
    ("coverage_threshold", connectivity_requirement,)
    ("required_points", (spawn_point,)),
    ("carve_radius", 1),
    ("use_mst_terminals", true),
]);

let mut pipeline = Pipeline::new();
pipeline.add_operation(pipeline::Operation::Algorithm { //No ConditionalOperation::simple(
        name: "cellular".to_string(),
        seed: rng.random(),
        config: cellular_config,
    }); //auto index : 0
pipeline.add_operation(ConditionalOperation::conditional_if{ //only has condition, if_true, if_false
        //Check if how much of the mape the spawnpoint is connected to
        condition: PipelineCondition::Connectivity {
            min: connectivity_requirement,
            terminal: spawn_point, //
        }, // this should update some parameter in the context
        if_true: vec![ //if_true
            pipeline::Operation::Log {
                message: "Connectivity: PASS".to_string(),
            },
        ],
        if_false: vec![pipeline::Operation::Algorithm{
            name: "gsb".to_string(),
            seed: rng.random(),
            config: gsb_config,
        }]
}); //auto index : 1
pipeline.add_operation(pipeline::Operation::Algorithm { //No ConditionalOperation::simple(
        name: "bsp".to_string(),
        seed: rng.random(),
        //default config
    },1); //Index to insert at
// Pipeline execution cellular->bsp->(if connectivity fail -> gsb)->
// operations: [cellular,bsp,conditinal_if]
let mut grid = Grid::new(40, 30);
let mut context = PipelineContext::new();

let result = pipeline.execute(&mut grid, &mut context, &mut rng);
```

(Should not have to include conditional should also be for just piping algorithms together)

Or

```rust
//Same variables as above
let mut pipeline = Pipeline::new();
pipeline.add_algorithm("cellular", rng.random(), cellular_config);
pipeline.add_conditional_if(
    PipelineCondition::Connectivity {min: connectivity_requirement,terminal: spawn_point},
    vec![ pipeline::Operation::log("Connectivity: PASS") ],
    vec![ pipeline::Operation::algorithm("gsb", rng.random(), gsb_config) ]
);
pipeline.add_algorithm("bsp", rng.random(), None, 1);//Index to insert at
let mut grid = Grid::new(40, 30);
let mut context = PipelineContext::new();
let result = pipeline.execute(&mut grid, &mut context, &mut rng);
```

(Should be easy to use and not require a bunch of boilerplate)

Layered generation:

```rust
let mut rng = Rng::new(seed);
let mut grid_1 = Grid::new(80, 60);
let mut grid_2 = Grid::new(80, 60);
let mut grid_3 = Grid::new(80, 60);
algorithms::exec("bsp",&mut grid_1, rng.random());
algorithms::exec("percolation",&mut grid_2, rng.random());
algorithms::exec("percolation",&mut grid_2, rng.random());
algorithms::combine(CombineMode::Union,&mut grid_1, &grid_2);
algorithms::exec("invert",&mut grid_3, None);
algorithms::combine(CombineMode::Difference,&mut grid_1, &grid_3);
```

Or (I don't know if this is better or worse)

```rust
let mut rng = Rng::new(seed);
let mut grid_1 = Grid::new(80, 60);
let mut grid_2 = Grid::new(80, 60);
let mut grid_3 = Grid::new(80, 60);
let mut grid_4 = Grid::new(80, 60);
//Does some algorithm execution
let mut output_grid_1 = algorithms::combine("union",&grid_1, &grid_3); //Could parse into CombineMode::Union in the combine function
let mut output_grid_2 = algorithms::combine("difference",&grid_1, &grid_4);
algorithms::exec("erode",&mut output_grid_2, rng.random());
let mut output_grid_3 = algorithms::combine("intersect",&output_grid_1, &output_grid_2);
```

### Additional Notes

1. Having to get("bsp").unwrap().generate(...) is cumbersome. There should be a easy way to just do algorithms::exec("bsp",...).
2. Configure parameters should be easy to do either by passing in a config struct or a list of key-value pairs.
3. To make it easier to use blend, effects, and other such operations should be available through algorithms::exec as well.
4. If two grids are used they should be available through algorithms(grid1,grid2,mode).
5. Seed and config should be optional parameters for algorithms that don't need them.
   e.g.

```rust
let resize_config = HashMap::from([
    ("width",100),
    ("height",100),
    ("padding_value",0),
]);
let erode_config = HashMap::from([
    ("iterations",3),
]);
let dilate_config = HashMap::from([
    ("iterations",2),
]);
let warp_config = HashMap::from([
    ("magnitude",5.0),
    ("frequency",0.1),
]);
let bridge_config = HashMap::from([...]);
let clear_config = HashMap::from([
    ("value",0),("shape","circle"),("radius",5)]);
let carve_path_config = HashMap::from([...]);
let mirror_config = HashMap::from([
    ("axis","vertical")]);
let rotate_config = HashMap::from([
    ("angle",90)]);
let scatter_config = HashMap::from([...]);
let fill_regions_config = HashMap::from([
    ("fill_mode","unreachable"),
    ("from",spawn_point),
]);

algorithms::exec("bsp",&mut grid_1, seed,bsp_config);
//All currently implemented algorithms...

// Unify usage pattern
algorithms::exec("invert",&mut grid_1,None,None); // Seed and config optional
algorithms::exec("resize",&mut grid_1,None,resize_config);
algorithms::exec("erode",&mut grid_1,None,erode_config);
algorithms::exec("dilate",&mut grid_1,None,dilate_config);
algorithms::exec("open",&mut grid_1,None,open_config);
algorithms::exec("close",&mut grid_1,None,close_config);
algorithms::exec("fill_regions",&mut grid_1,None,fill_regions_config);

algorithms::exec("warp",&mut grid_1,None,warp_config);

algorithms::exec("bridge",&mut grid_1,None,bridge_config);
algorithms::exec("clear",&mut grid_1,None,clear_config);
algorithms::exec("carve_path",&mut grid_1,None,carve_path_config);

algorithms::exec("mirror",&mut grid_1,None,mirror_config);
algorithms::exec("rotate",&mut grid_1,None,rotate_config);
algorithms::exec("scatter",&mut grid_1,None,scatter_config);

algorithms::combine("union", &mut grid_1, &grid_2); //CombineMode::Union
algorithms::combine("difference", &mut grid_1, &grid_3); //CombineMode::Subtract
algorithms::combine("intersect", &mut grid_1, &grid_4); //CombineMode::Intersect
// Maby other combine modes as well

```

## Example that should be easy to implement

```text
Floor map:
    grid_1: simplex-fill Noise (Multi-octave) threshold=0.3, (loose sand)
    grid_2: simplex-fill Noise (Multi-octave) threshold=0.7 (same seed as grid_2) (desert pavement)

    grid_1 = grid_1 - grid_2  //could be done by just overwriting when adding grid_2 to combination map

    Combination map (integer values):
        0 = salt encrusted sand # entire grid (salt encrusted sand)
        1 = loose sand
        2 = desert pavement
    return combination map
```

```text
Wall map:
    grid_1 (Inselbergs (Isolated Rock Islands)):
        Use Voronoi Diagrams to pick "seed points" where an inselberg might exist.
        Around these points, apply 3–5 iterations of Cellular Automata to grow a jagged, rocky cluster. This creates the "island of rock" look surrounded by flat basins.
        Apply a strict Masking step: walls only spawn if the "Floor Layer" it intersects with floor_map[x,y]==2
    grid_1 (Salt Outcrops & Ranges):
        Use Perlin Ridge Noise (taking the absolute value of the noise) to create thin, sharp "ranges" that mimic wind-carved stone or jagged salt ridges.
        Apply a strict Masking step: walls only spawn if the "Floor Layer" it intersects with floor_map[x,y]==0 noise is below a certain value (representing the lower basins where sediment hasn't covered the rock).
    Combination map (integer values):
        0 = empty space
        2 = sandstone (inselbergs)
        3 = saltstone (outcrops & ranges)
    return combination map
```
