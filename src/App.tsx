import { invoke } from '@tauri-apps/api/tauri'
import { Component, Suspense, createResource, createSignal } from 'solid-js'
// import { ApiData } from './util/data'

const fetch_data = async (q: string): Promise<string> => {
    return await invoke<string>('search_api', { query: q })
        .then(res => res)
}

const set_jikan = async () => await invoke('set_api_impl', { implName: 'Jikan (3rd party MyAnimeList API)' })
const set_kitsu = async () => await invoke('set_api_impl', { implName: 'Kitsu.io' })

const App: Component = () => {
    const [query, setQuery] = createSignal('sword art online')
    const [data, { refetch }] = createResource(query, fetch_data)

    return (
        <div>
            <input type="text" onChange={(e) => setQuery(e.currentTarget.value)} />
            <button onClick={refetch}>refresh</button>
            <button onClick={set_kitsu}>Kitsu</button>
            <button onClick={set_jikan}>Jikan</button>
            { data.loading ? <p>Loading...</p> : 
                <div>
                    <pre>{ JSON.stringify(data(), null, 2) }</pre>
                </div> 
            }
            {/* <Suspense fallback={<p>Loading...</p>}>
                <pre>{ data()?.anime?.[0].titles.en }</pre>
                <For each={data()?.[0].data}>
                    {(item, index) => 
                        <div>
                            #{index()} {item.attributes.titles.en}
                        </div>
                    }
                </For>
            </Suspense> */}
        </div>
    )
}

export default App
