import { invoke } from '@tauri-apps/api/tauri'
import { Component, For, createResource, createSignal } from 'solid-js'
import { ApiData } from './util/data'
// import { Poster } from './components/poster'

const fetch_data = async (q: string): Promise<ApiData> => {
    return await invoke<ApiData>('search_api', { query: q })
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
                // <div>
                //     <pre>{ JSON.stringify(data(), null, 2) }</pre>
                // </div> 
                <div id="list">
                    <For each={data()?.anime}>
                        {(item, index) => 
                            // <Poster data={item} />
                            <div id="list__item">
                                <h3>{ item.canon_title || item.titles.en || item.titles.jp }</h3>
                                <p>Rank { item.rank } / Rating { item.rating }</p>
                                <p>Aired { item.start_date } to { item.end_date || 'TBA' }</p>
                            </div>
                        }
                    </For>
                    <For each={data()?.manga}>
                        {(item, index) => 
                            // <Poster data={item} />
                            <div id="list__item">
                                <h3>{ item.canon_title || item.titles.en || item.titles.jp }</h3>
                                <p>Rank { item.rank } / Rating { item.rating }</p>
                                <p>Aired { item.start_date } to { item.end_date || 'TBA' }</p>
                            </div>
                        }
                    </For>
                </div>
            }
        </div>
    )
}

export default App
