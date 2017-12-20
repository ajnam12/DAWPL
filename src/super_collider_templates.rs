macro_rules! k_instruments {
// Synth from this video: "https://www.youtube.com/watch?v=nB_bVJ1c1Rg"
    () => ("
(
SynthDef.new(\\sine, {
    arg freq=440, atk=0.005, rel=0.3, amp=1, pan=0;
    var sig, env;
    sig = SinOsc.ar(freq);
    env = EnvGen.kr(Env.new([0, 1, 0], [atk, rel], [1, -1]), doneAction:2);
    sig = Pan2.ar(sig, pan, amp);
    sig = sig * env;
    Out.ar(0, sig);
}).add;
)
")
}

macro_rules! k_instrument_template {
    () => ("
{var_name} = Pbind(
    \\instrument, \\{instrument_name},
    \\dur, Pseq({dur}),
    \\midinote, Pseq({midi_notes}),
);
")
}

macro_rules! k_audio_file_template {
    () => ("
{var_name} = {{PlayBuf.ar(2, Buffer.read(s, {filepath}))}};
")
}

macro_rules! k_empty_clip_template {
    () => ("
{var_name} = (note:Rest(), dur:{dur});
")
}

// Pseq([v1, v2, .., vN]).do({arg thing; thing.play}).play (sequencing)

macro_rules! k_track_template {
    () => ("
{track_name} = Pseq({clips}).do({{arg currClip; currClip.play}});
")
}

// [v1, v2, ..., vN].do({arg thing; thing.play})

macro_rules! k_arrangement_template {
    () => ("
{instruments}
(
{variable_declarations}
{clip_declarations}
{track_declarations}
{track_names}.do({{arg currTrack; currTrack.play}})
)")
}