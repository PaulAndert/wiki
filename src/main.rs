use leptos:: *;

// run with
// trunk serve --open   

#[derive(Default, Clone, Debug)]
struct GlobalVariables {
    state: u32,
    search_string: String,
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

// manpages DB table
// id
// title
// path (with extension)

// maybe table related manpages
// 

#[component]
fn App(cx: Scope) -> impl IntoView {
    let base_path: String = String::from("/Users/aya/Documents/manpages/");

    let global: RwSignal<GlobalVariables> = create_rw_signal(cx, GlobalVariables::default());
    provide_context(cx, global);

    let (get_state, set_state): (Signal<u32>, SignalSetter<u32>) = create_slice(
        cx, global, |global| global.state, 
        |global: &mut GlobalVariables, n| global.state = n,
    );
    let (get_search_string, _set_search_string): (Signal<String>, SignalSetter<String>) = create_slice(
        cx, global, |global| global.search_string.clone(), 
        |global: &mut GlobalVariables, n| global.search_string = n,
    );
    
    view! { cx,

        <button on:click=move |_| { set_state(1)} >
        "Click me: 1"
        </button>
        <button on:click=move |_| { set_state(2)} >
        "Click me: 2"
        </button>
        <button on:click=move |_| { set_state(get_state() + 1)} >
        "Increment"
        </button>

        <main>
        {move || match get_state() {
            1 => view! { cx, 
                <div> <SearchComponent/> </div> 
            },
            2 => view! { cx, 
                <div> <FormComponent/> </div> 
            },
            _ => view! { cx, 
                <div>{ get_state() }</div> 
            },
        }}
        </main>

        <p> {get_search_string} </p>

    }
}

#[component]
fn ProgressComponent(cx: Scope) -> impl IntoView {
    let get_state = use_context::<ReadSignal<i32>>(cx)
        .expect("to have found get_state provided");
    view! { cx,
    <progress
        max="20" value=move || get_state.get() * 2 />
    }
}

#[component]
fn SearchComponent(cx: Scope) -> impl IntoView {
    let global = use_context::<RwSignal<GlobalVariables>>(cx).expect("to have found global provided");

    let (get_search_string, set_search_string) = create_slice(
        cx, global, |global| global.search_string.clone(), 
        |global: &mut GlobalVariables, n| global.search_string = n,
    );

    view! { cx,
        <input type="text"
            on:input=move |ev| {
                set_search_string(event_target_value(&ev));
            }
            prop:value=get_search_string
        />
    }
}


#[component]
fn FormComponent(cx: Scope) -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;
    use leptos::ev::SubmitEvent;

    let (name, set_name) = create_signal(cx, "".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = input_element()
            .expect("<input> to exist")
            .value();
        set_name(value);
    };

    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}