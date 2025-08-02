<script lang="ts">
    import { onMount } from "svelte";
    let query = "0.1, 0.2, 0.3, 0.4";
    let result: any = null;
    let loading = false;
    let error = "";

    async function doSearch() {
        loading = true;
        error = "";
        result = null;
        try {
            const response = await fetch("http://localhost:8080/search", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ vector: query.split(",").map(Number) }),
            });
            if (!response.ok) throw new Error(await response.text());
            result = await response.json();
        } catch (err) {
            error = String(err);
        } finally {
            loading = false;
        }
    }
</script>

<h1>Vector Search Demo</h1>
<input bind:value={query} placeholder="Enter comma-separated vector" style="width: 300px;">
<button on:click={doSearch} disabled={loading}>Search</button>

{#if loading}
    <p>Searching…</p>
{:else if error}
    <p style="color: red">{error}</p>
{:else if result}
    <h2>Results</h2>
    <pre>{JSON.stringify(result, null, 2)}</pre>
{/if}
