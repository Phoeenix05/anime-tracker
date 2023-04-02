import { invoke } from '@tauri-apps/api/tauri'
import { Component, For, Suspense, createResource, createSignal } from 'solid-js'
import { JikanData, KitsuData } from './util/data'

// await invoke('set_api_impl', { implName: 'Jikan (3rd party MyAnimeList API)' })
// const data = await invoke('get_api_impls', {})
// console.log(data)

const fetch_data = async (q: string): Promise<[KitsuData, KitsuData]> => {
    return await invoke<[string, string]>('search_api', { query: q })
        .then(res => [JSON.parse(res[0]), JSON.parse(res[1])])
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
            {/* <Suspense fallback={<p>Loading...</p>}>
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
