const p = Deno.run({
    cmd: ["ls"],
});

await p.status();