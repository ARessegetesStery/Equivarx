# Rendering Roadmap

The plan for building Equivarx's renderer and for teaching the owner the graphics behind it, driven by what the game Occluscence has to show. An implementing or teaching LLM should be able to start from this file, the governing instruction file (`CLAUDE.md` or `AGENTS.md`) and the project memory, with no other context. This is a living document: revise it in place as stages complete or decisions change, and keep history in `docs/memory/` and `docs/lodestar/`.

Paths starting with `occ/` are in the game's repository, `C:\_azima\occluscence\occ`, which is outside this repository and may not be present on every machine. The owner-written design document is `occ/docs/design/design-document.md`. Its underscore-prefixed siblings were written by an LLM and are not reliable for specific mechanics.

## 1. Scope

This roadmap covers surfaces, light, post-processing and mist in both of the game's worlds. Game engine systems (job system, world model, gameplay, collision, audio, moving the game in) belong to a separate engine roadmap that has not been written yet; Section 8 lists what it inherits.

The end state is one continuous camera route that runs on budget and looks right: from an overlook across a landscape of mist, into a building, up to a mist wall, across the threshold, and through the mist side looking back at the physical world.

## 2. Settled decisions

Settled with the owner on 2026-11-01. Reopen one only if a measurement or a design change contradicts its reasoning.

| Decision | Reasoning |
|---|---|
| Rendering comes first; the engine roadmap takes its place after Stage 5 | Rendering holds the hardest planned features and most of what the owner wants to learn. The CPU has little to do yet: the game's spine and shroud cost about 0.1 ms per tick at 5x12 (`occ/docs/memory/shroud-next-steps.md`) |
| Target is the owner's machine: RTX 4070 Laptop GPU, i9-14900HX, 1080p, 60 fps | The project is for fun rather than for release (`occ/docs/memory/project-direction-for-fun.md`), and supporting weaker GPUs would rule out techniques |
| All mist work shares about 5 ms of the 16.6 ms frame | Mist is the game's principal technical differentiator, but surfaces and post-processing also need room. Section 5 splits it |
| Close mist inside architecture and landscape-scale far mist are both first-class | Both are essential to the game, and they are different problems (Section 4) |
| The two worlds are two different rendering problems | They occupy the same space, but most rendering configuration differs between them. Do not design the mist side as a tint on the physical world |
| The look is physically grounded, with deliberate breaks | This keeps a path-traced reference meaningful for checking and teaching. One-way transparency is the first deliberate break, because the design requires it and physical light transport cannot produce it |
| Nothing is built beyond what Occluscence needs | No custom editor or general-purpose engine features: Blender for content, an in-game inspector for tuning. This keeps the target content feasible for one person |
| No mist rendering in Godot; the owner experiments with time mechanisms there | Mist shaders written against Godot would be thrown away. Findings from the time experiments feed Stages 5 and 7 |
| Each concept is taught in conversation with the math derived, plus one or two primary sources | Owner's choice of learning format. Section 3 gives the procedure |
| The graphics API stays wgpu | See `docs/memory/engine-direction.md`. Propose something lower-level only if an algorithm needs what wgpu does not expose |
| The owner owns design and predictions; agents write the code | See `docs/memory/engine-direction.md`. Do not expect the owner to hand-write code, and bring design decisions to them rather than settling those alone |

## 3. How each stage runs

The owner has a strong physics simulation background, and their graphics knowledge is about four years old and rusty. Teach accordingly: lean on the physics and the math, and do not assume current graphics vocabulary.

1. **Teach.** Work through the stage's Learn list in conversation, deriving the math rather than stating results. Explain conceptually and intuitively, and cite one or two primary sources after the explanation. Stop when the owner can explain the tradeoffs back.
2. **Decide.** Bring each Owner decides item to the owner with options and a recommendation. Numbers marked provisional in this file are set here.
3. **Predict.** Before anything is built, the owner writes down predictions, using the stage's prompts as a start. These go into the stage's dated record under `docs/lodestar/`.
4. **Build.** Agents implement. Every task that can be checked through the bench (Stage 0) is checked that way without the owner watching.
5. **Compare.** Measure against the predictions and exit criteria. Treat each surprise as the next thing to teach.
6. **Close.** The owner rules on the judged criteria by looking at the result. Then revise this file in place and update the project memory.

Exit criteria are either measured, which an agent can verify alone, or judged, which only the owner can rule on.

## 4. What the renderer has to show

| | Close | Far |
|---|---|---|
| **Physical world** | Mist inside architecture: rooms, corridors, rails, and walls at arm's length, with the camera beside or inside it. Sharp detail and occlusion by walls dominate | Landscape-scale mist and the sky: "the upper dome will be covered by mist, and what is above that cannot be seen at all". Large form and soft multiple scattering dominate, fine detail is invisible, and it changes slowly |
| **Mist side** | Not yet designed. It must read "as a world rather than as an effect", and it includes the mirrored hub room of boards and notes | Seeing through the mist to the physical world, upside down and replaying in reverse, descending a tower. Turbulence authored by every stabilizer the player placed |

Requirements from the design documents, and what each one implies:

- **The mist wall is a boundary layer.** It is "a violent interface", where opposed flows meet. Coarse grid fog cannot hold that detail at arm's length, so close mist needs a sharper technique over the grid.
- **One-way transparency.** "The mist can be seen through from within, and not from without." Physical light transport is the same in both directions, so what a ray sees has to depend on where the camera is. The change must also read correctly as the camera crosses.
- **Density must be readable.** Thick, thin and artifact-attenuated mist have to look different (`occ/docs/Prototypes/P1.md`), and a rail's density has to be "judgeable by eye", because it tells the player the cost of a traversal. A stabilizer's protection radius needs clear visual indication.
- **The mist is alive.** It "should breathe, respond to proximity", and be "a reactive, physically simulated system".
- **The player is translucent and luminous.** The shroud is an emissive, translucent shell with swirling currents inside (`occ/docs/memory/shroud-render-mesh-design.md`), and it moves through the mist. Translucent surfaces need correct fog, and the shroud should light the mist.
- **The crossing is continuous.** "No loading screen, a slow atmospheric dissolve." The two worlds' configurations must blend, not switch.
- **Dizziness is perceptual.** It is "failure to record", shown as "perceptual and atmospheric deterioration, never a meter".
- **Geometry is sparse.** The art direction is low visibility, silhouettes and procedural generation. Surfaces are a small share of the work, so techniques for dense geometry (virtual geometry, heavy culling) are out of scope.

Close and far mist also meet: a wall seen from an overlook is the same wall the player later stands beside. Two representations must agree on density, color and lighting where they hand over, and that handover is a hard problem in its own right.

## 5. Frame budget

Provisional starting points for a 16.6 ms frame, measured on the target machine on mains power. Stage 2 replaces them with measurements.

| Slice | ms |
|---|---|
| Surfaces and shadows | 4.0 |
| Close mist | 3.0 |
| Far mist and sky, averaged across frames | 1.5 |
| Mist simulation | 0.5 |
| Temporal anti-aliasing and post-processing | 2.0 |
| Headroom for spikes, thermal limits and CPU work | 5.6 |

The slices add up rather than overlap. wgpu exposes a single queue with no asynchronous compute, so simulation and rendering run one after the other. The newest evidence is a third-party audit of wgpu 29 (Bloom-Engine/engine#29; the tracking issue is gfx-rs/wgpu#1066); recheck when upgrading in Stage 0.

## 6. Rules that span stages

- **Rasterize surfaces; raymarch mist in compute shaders.** Hardware ray tracing is not needed, and wgpu offers it only as an experimental feature.
- **The renderer is a function of a per-frame description.** It receives the camera, lights, mesh instances, mist field descriptions and time, and knows nothing about game state. Test replays fill the description now, and the engine roadmap fills it later. Reversed replay in Stage 6 then becomes feeding recorded descriptions backward.
- **GPU results never drive gameplay.** GPU simulation may differ across hardware and reading it back adds lag, either of which would break the game's deterministic replay (`occ/Impl-Doc.md`, cross-file rules). Gameplay reads a CPU-owned field, which the engine roadmap provides; GPU detail is visual only.
- **One reference route.** A single camera path through one test world grows stage by stage: overlook, approach, interior, wall, crossing, mist side. Each stage adds its segment, and earlier segments become regression tests.
- **The shroud arrives as recorded data, not as a port.** The game's harness already writes JSONL trajectories (`occ/game/utils/test_tools/headless_harness/trajectory_recorder.gd`) with per-tick `player`, `spine` and `shroud` positions, in ring-major grid order `r * C + c`. Replaying these gives real motion and keeps this roadmap independent of the engine roadmap.
- **Separate until exactly matched.** Close and far mist, and the physical and mist-side configurations, keep separate code until a second consumer needs exactly the same operation. See `docs/memory/single-consumer-code-stays-put.md`.
- **Code placement.** Deep rendering algorithms such as mist go in `eqx_render`. Window and device setup stay in `eqx_window`.

## 7. Stages

| Stage | Part | Result |
|---|---|---|
| 0. The bench | A | A renderer that can be measured without anyone watching |
| 1. Light and the image chain | A | A dark, deliberate-looking room with complete post-processing |
| 2. Light in a medium | A | Grid fog checked against a path-traced reference |
| 3. Close mist | A | A mist wall that holds up at arm's length |
| 4. Far mist | A | Vistas and the dome, with seamless handover to close mist |
| 5. Living mist and the shroud | A | Mist that moves and reacts, with the shroud inside it |
| Engine roadmap | -- | Separate document (Section 8) |
| 6. The mist side | B | The second world, and a continuous crossing |
| 7. Perception and time | B | Dizziness and time-reversal visuals |

Part A runs first. Stages 4 and 5 both build on Stage 3 but not on each other, so they can swap if the owner prefers. Part B follows the engine roadmap because it consumes what the engine provides: recorded world state and working time mechanics.

### Stage 0: The bench

**Goal.** A renderer that agents can run and measure alone and the owner can iterate on live, replacing the current tutorial-style single `State` in `crates/eqx_window/src/window_display.rs`.

**Learn.** How a frame reaches the screen: command encoding, render and compute passes, vertex to rasterizer to fragment, depth, how GPU timing differs from CPU timing. What wgpu handles automatically compared with Vulkan or DirectX 12 (resource barriers, memory). Sources: Giesen; RTR4 chapters 2 and 3.

**Build.**
- Upgrade wgpu (currently 26) and winit (currently 0.29) to current releases.
- Separate device and surface ownership from the renderer, and give the renderer its per-frame description.
- A frame loop sufficient for rendering work. `Module::init` currently blocks forever and `Module::update` never runs; the full simulation loop belongs to the engine roadmap.
- A small declared pass list: order, inputs and outputs, and screen-size-dependent textures. wgpu handles barriers, so this stays small.
- WGSL hot reload and a way to share shader code between files.
- GPU timestamps per pass.
- Headless mode: offscreen target, scripted camera, fixed clock and frame sequence, writing PNG images and per-pass timings as JSON.
- An image comparison tool for later stages.
- An in-window inspector for live parameters and debug views.
- The first segment of the reference route, in a stand-in scene.

**Exit.** Measured: one command writes images and per-pass timings; rerunning a static scene reproduces the images within tolerance; a shader edit applies without restarting. Judged: the owner can capture a frame in a GPU debugger and read the passes.

**Predict.** Which pass costs the most? How do timings change from 1080p to 540p?

**Owner decides.** How much structure the pass list has; which metric the comparison tool reports.

### Stage 1: Light and the image chain

**Goal.** Correct light on sparse surfaces, and a complete image chain for mist to plug into. Temporal anti-aliasing belongs here because Stages 2 and 3 depend on reusing samples across frames.

**Learn.** Radiance and irradiance; diffuse and GGX specular reflection; HDR and exposure; shadow maps, cascades and filtering; tonemapping; bloom; temporal anti-aliasing and motion vectors; reversed depth for kilometer ranges. Sources: RTR4 chapters 7 to 9; Pettineo; Karis; Jimenez; Gjoel and Svendsen.

**Build.**
- The test world: a building with thin walls, windows and a long sightline outdoors.
- A forward surface pass with a depth prepass, one directional light and a few shadowed local lights.
- An HDR target, exposure, tonemapping, bloom and dithering against banding.
- Temporal anti-aliasing with jitter and motion vectors.
- A placeholder HDR sky, and reversed depth with a far plane suited to landscapes.

**Exit.** Measured: surfaces and post-processing stay within budget on the route so far. Judged: a dark silhouette room looks deliberate, and camera motion shows no distracting ghosting.

**Predict.** Where does banding appear first? How far can shadow cascades reach before the resolution shows?

**Owner decides.** Tonemapper; forward or deferred (recommended: forward, since lights are few and translucency matters).

### Stage 2: Light in a medium

**Goal.** The theory, a ground truth to check against, and working grid fog.

**Learn.** The radiative transfer equation: absorption, out-scattering, in-scattering, emission. Extinction, optical depth, Beer-Lambert transmittance, albedo. Phase functions such as Henyey-Greenstein. Single against multiple scattering. Monte Carlo free-path sampling, delta tracking and ratio tracking. Then camera-fitted 3D grids ("froxels"): depth slice distribution, energy-conserving integration, reuse across frames. Sources: PBRT chapters 11 and 14; Fong et al.; Novak et al.; Wronski; Hillaire 2015; RTR4 chapter 14.

**Build.**
- A progressive path tracer in a compute shader: delta tracking, multiple scattering, offline accumulation. It never ships; it is the answer key.
- Fixtures: uniform fog, height fog, a sunbeam through a window, and dense mist beside a thin wall.
- The grid pipeline: inject density, light each cell using shadow maps, reuse across frames, integrate front to back, apply to surfaces and translucent objects.
- A report of grid error against the reference for each fixture.

**Exit.** Measured: thin-fog error stays within the agreed bound; the errors from missing multiple scattering and from light leaking through thin walls are measured and recorded (both are addressed in Stage 3); cost stays within the close-mist budget; the budget table is updated. Judged: light shafts look right.

**Predict.** What does doubling the depth slices do to error and cost? At what optical thickness does single scattering visibly fail? How many frames of reuse does the fog need before it settles?

**Owner decides.** Grid resolution; phase function parameters; the error metric and its bound.

### Stage 3: Close mist

**Goal.** Physical-world mist inside architecture that holds up at arm's length.

**Learn.** Raymarching; jittered sampling and blue noise; reprojecting volumes across frames; depth-aware upsampling; designing noise for density; self-shadowing; approximating multiple scattering; why thin walls leak and what prevents it. Sources: Wrenninge et al.; Schneider and Vos; Gjoel and Svendsen; Wolfe et al.

**Build.**
- A half-resolution raymarch for dense structures, composited with the grid fog for thin ambient mist.
- Density derived from an authored signed field, so the visible wall is a boundary layer.
- Thick, thin and artifact-attenuated states, a visible stabilizer radius, and a dense rail channel.
- One-way transparency, including the change as the camera crosses.
- A fix for wall leaks, tested with the Stage 2 fixture.

**Exit.** Measured: cost stays within budget on the interior and wall segments; the leak fixture meets its bound. Judged: standing 1 m from the wall still rewards looking; crossing from outside to inside reads as intended; density can be judged by eye.

**Predict.** Which reduces error more per millisecond, more steps or higher resolution? Where does reprojection ghost under fast camera motion?

**Owner decides.** The wall's visual target (reference images); how one-way transparency works; how the stabilizer zone looks.

### Stage 4: Far mist

**Goal.** Landscape-scale mist and the overhead dome, with seamless handover to close mist.

**Learn.** Sky and aerial perspective; large-scale form from coverage maps and eroded noise, 2.5D against voxel representations; why multiple scattering dominates the look at this scale; forward scattering and bright edges; spreading updates across frames; mist shadows on terrain. Sources: Schneider and Vos; Schneider 2017; Schneider 2023; Bauer; Hillaire 2016; Hillaire 2020.

**Build.**
- The overlook segment with a terrain heightfield.
- Sky and dome, with aerial perspective.
- A low-resolution vista raymarch with updates spread across frames, and mist shadows cast on the land.
- Authoring in which one placed mist mass appears at both scales.
- The handover between far and close representations.

**Exit.** Measured: average cost stays within budget without spikes; on a continuous approach from about 2 km to 1 m from a wall, frame-to-frame luminance jumps stay within the agreed bound. Judged: the overlook conveys place and scale, and no pop is visible during approach.

**Predict.** At what distance does the handover become invisible? How many frames can an update spread over before the delay shows in motion?

**Owner decides.** The dome's look; a painted or computed sky; how far vistas reach; authored or procedural mist placement.

### Stage 5: Living mist and the shroud

**Goal.** Mist that moves and reacts, with the player's shroud living inside it.

**Learn.** Semi-Lagrangian advection, pressure projection, numerical dissipation, vorticity; curl noise; low-resolution simulation with procedural detail; translucent surfaces inside volumes; volumetric emission; which of these methods can run backward. Sources: Stam; Bridson et al.; Crane et al.; Bridson's book.

**Build.**
- The shroud shell from recorded trajectories, with emission and interior currents, sampling fog at its own depth and lighting the mist.
- Mist motion from a curl-noise flow.
- A local GPU fluid grid that follows the camera, so a moving shroud parts the mist and leaves a wake.
- Slow large-scale evolution for far mist.
- Time direction as the first open question in Section 9 decides.

**Exit.** Measured: all mist stays within budget with the shroud present. Judged: the mist breathes, the wake persists believably, and the shroud reads as luminous and belonging in the fog.

**Predict.** How coarse can the grid get before a wake turns to blur? How many pressure iterations are needed before the mist stops looking compressible?

**Owner decides.** How the mist reacts; time direction; the shroud's look.

### Stage 6: The mist side

**Goal.** The second rendering problem. This stage does not start until the owner has set a visual target, because most of it is not yet designed.

**Learn.** Set at the stage's start. Likely topics are viewing a world through an inverting transform, rendering a whole world at a distance within budget, and blending between rendering configurations.

**Build (provisional).**
- A separate rendering configuration for the mist side.
- The inverted physical world seen through the mist, replaying recorded frame descriptions in reverse.
- Turbulence authored from stabilizer placement.
- The crossing, as a continuous dissolve between configurations.

**Exit (provisional).** Measured: the crossing has no cut, and frame-time spikes stay within the agreed bound. Judged: it reads "as a world rather than as an effect", and the backward tour is a vivid image rather than a backdrop.

**Predict.** What does seeing both worlds at once cost, and which part of it can be amortized?

**Owner decides.** The visual target; how detailed the inverted physical world must be.

### Stage 7: Perception and time

**Goal.** The visual effects of the body and of time, shaped by the owner's time experiments in Godot.

**Learn.** Depth of field and blur; effects built on frame history; how to show a process running backward legibly.

**Build (provisional).**
- The dizziness deterioration ("failure to record"), possibly using frame history.
- Reverse-time visuals for the processes the experiments settle on, such as water or fire.
- Final grading for each world.

**Exit.** Judged: dizziness reads as failing perception rather than a filter, and reversal is legible. Measured: the full reference route runs on budget, which completes this roadmap.

**Predict.** Set at the stage's start.

**Owner decides.** Which effects the time experiments call for.

## 8. Handed to the engine roadmap

The engine roadmap replaces the earlier idea of a parallelism stage here. It is large work that includes learning more Rust. It inherits:

- The full frame loop: fixed simulation tick, variable-rate rendering, interpolation.
- The job system.
- Copying render data off the simulation thread to fill the per-frame description.
- The world model, including whether to use an ECS.
- The CPU-owned time-rate field for gameplay, per-object time rates, and state history for reversal.
- Porting the tail, spine, shroud and player to Rust, using recorded trajectories from the game's harness to check equivalence.
- Collision, audio, note text, and the level pipeline (Blender with an in-game inspector).
- Moving Occluscence in. The first scene to aim for is the design document's prototype: one room, one mist boundary, one stabilizer.

## 9. Open questions

1. **Does mist seen from outside visibly run backward?** The design says processes inside run backward. If that includes the mist's own motion, a forward fluid solver cannot simply be reversed, because its numerical dissipation cannot be undone. That favors analytic flow, which can run in either direction, or playing a recorded simulation backward. It also raises a question about reactions: does mist part before the player arrives? Decide before Stage 5, using the Godot time experiments.
2. **What exactly is visible from outside a wall?** The boundary layer is what the design says is visible, and the first scene describes a "half-opaque boundary". Whether anything shows through it, and when, shapes the one-way transparency model. Decide in Stage 3.
3. **What is the mist side's visual target?** It has no design yet beyond the requirements in Section 4. Decide before Stage 6.
4. **Do the time experiments change what the renderer needs?** Revise Stages 5 to 7 as findings arrive.

## 10. Sources

- Akenine-Moller et al., *Real-Time Rendering*, 4th edition, 2018 (RTR4).
- Bauer, "Creating the Atmospheric World of Red Dead Redemption 2: A Complete and Integrated Solution", SIGGRAPH 2019, Advances in Real-Time Rendering.
- Bridson, *Fluid Simulation for Computer Graphics*, 2nd edition, 2015.
- Bridson, Houriham and Nordenstam, "Curl-Noise for Procedural Fluid Flow", SIGGRAPH 2007.
- Crane, Llamas and Tariq, "Real-Time Simulation and Rendering of 3D Fluids", *GPU Gems 3*, chapter 30, 2007.
- Fong, Wrenninge, Kulla and Habel, "Production Volume Rendering", SIGGRAPH 2017 course.
- Giesen, "A Trip Through the Graphics Pipeline 2011", blog series.
- Gjoel and Svendsen, "Low Complexity, High Fidelity: INSIDE Rendering", GDC Europe 2016. A close reference for the game's low-visibility look: https://gdcvault.com/play/1023783/Low-Complexity-High-Fidelity-INSIDE
- Hillaire, "Physically Based and Unified Volumetric Rendering in Frostbite", SIGGRAPH 2015, Advances in Real-Time Rendering: https://www.ea.com/news/physically-based-unified-volumetric-rendering-in-frostbite
- Hillaire, "Physically Based Sky, Atmosphere and Cloud Rendering in Frostbite", SIGGRAPH 2016, Physically Based Shading course.
- Hillaire, "A Scalable and Production Ready Sky and Atmosphere Rendering Technique", EGSR 2020.
- Jimenez, "Next Generation Post Processing in Call of Duty: Advanced Warfare", SIGGRAPH 2014, Advances in Real-Time Rendering.
- Karis, "High Quality Temporal Supersampling", SIGGRAPH 2014, Advances in Real-Time Rendering.
- Novak, Georgiev, Hanika and Jarosz, "Monte Carlo Methods for Volumetric Light Transport Simulation", Eurographics 2018 state-of-the-art report.
- Pettineo, "A Sampling of Shadow Techniques", blog post, 2013.
- Pharr, Jakob and Humphreys, *Physically Based Rendering*, 4th edition, 2023 (PBRT), free online: https://pbr-book.org/
- Schneider and Vos, "The Real-time Volumetric Cloudscapes of Horizon: Zero Dawn", SIGGRAPH 2015, Advances in Real-Time Rendering.
- Schneider, "Nubis: Authoring Real-Time Volumetric Cloudscapes with the Decima Engine", SIGGRAPH 2017, Advances in Real-Time Rendering.
- Schneider, "Nubis Cubed: Methods (and madness) to model and render immersive real-time voxel-based clouds", SIGGRAPH 2023, Advances in Real-Time Rendering: https://advances.realtimerendering.com/s2023/Nubis%20Cubed%20(Advances%202023).pdf
- Stam, "Stable Fluids", SIGGRAPH 1999.
- Wolfe et al., "Spatiotemporal Blue Noise Masks", EGSR 2022.
- Wrenninge, Kulla and Lundqvist, "Oz: The Great and Volumetric", SIGGRAPH 2013 talk.
- Wronski, "Volumetric Fog: Unified Compute Shader-Based Solution to Atmospheric Scattering", SIGGRAPH 2014, Advances in Real-Time Rendering.
- wgpu queue status: https://github.com/gfx-rs/wgpu/issues/1066 and https://github.com/Bloom-Engine/engine/issues/29
