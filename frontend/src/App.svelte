<script lang="ts">
    let query = "Find me a red dress";
    let result: Array<{ id: number; score: number }> = [];
    let loading = false;
    let error = "";

    // Example embedding: turn text into a fixed-size vector
    function embedTextToVector(text: string, dim = 4): number[] {
        // Simple hash trick; replace with real embedding for production
        const vec = Array(dim).fill(0);
        for (let i = 0; i < text.length; i++) {
            vec[i % dim] += text.charCodeAt(i) / 100;
        }
        // Normalize (optional)
        const norm = Math.sqrt(vec.reduce((sum, x) => sum + x * x, 0));
        return norm > 0 ? vec.map(x => x / norm) : vec;
    }

    async function doSearch() {
        loading = true;
        error = "";
        result = [];
        let vector: number[] = [];

        // Try to parse vector, fallback to embedding
        if (query.match(/^(\s*\d+(\.\d+)?\s*,?)+$/)) {
            vector = query.split(",").map(x => parseFloat(x.trim()));
        } else {
            vector = embedTextToVector(query, 4);
        }

        try {
            const res = await fetch("http://localhost:8080/search", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ vector }),
            });
            if (!res.ok) throw new Error(await res.text());
            result = await res.json();
        } catch (err) {
            error = String(err);
        } finally {
            loading = false;
        }
    }
</script>

<h1>🛒 E-commerce Vector Search</h1>
<input bind:value={query} placeholder="Type your query (words or comma-separated vector)" style="width: 350px; padding: 8px;">
<button on:click={doSearch} disabled={loading} style="margin-left: 10px;">
    {loading ? "Searching..." : "Search"}
</button>

{#if error}
    <p style="color: red; font-weight: bold;">{error}</p>
{/if}

{#if result.length}
    <h2>Search Results</h2>
    <table border="1" style="margin-top: 10px;">
        <thead>
            <tr><th>Product ID</th><th>Similarity</th></tr>
        </thead>
        <tbody>
            {#each result as r}
                <tr>
                    <td>{r.id}</td>
                    <td>{r.score.toFixed(4)}</td>
                </tr>
            {/each}
        </tbody>
    </table>
{:else if !loading && !error}
    <p>Enter a product description or vector to search.</p>
{/if}
