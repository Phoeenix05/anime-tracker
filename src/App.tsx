import { invoke } from '@tauri-apps/api/tauri'
import { Component, createResource, createSignal } from 'solid-js'

const fetch_data = async (q: string): Promise<[object, object]> => {
    return await invoke<[string, string]>("search_api", { query: q })
        .then(res => [JSON.parse(res[0]), JSON.parse(res[1])])
}

const App: Component = () => {
    const [query, setQuery] = createSignal("sword art online")
    const [data, { mutate, refetch }] = createResource(query, fetch_data)
    
    return (
        <div>
            <input type="text" onChange={(e) => setQuery(e.currentTarget.value)} />
            {/* <button onClick={refetch}>search</button> */}
            { data.loading ? <p>Loading...</p> : <div>
                <pre>{ JSON.stringify(data()?.[0], null, 2) }</pre>
                <pre>{ JSON.stringify(data()?.[1], null, 2) }</pre>
            </div> }
        </div>
    )
}

export default App
