use yew::prelude::*;

#[function_component(TodoList)]
pub fn todo_list() -> Html {
    html! {
      <ul class="todo-list">
        <li>
            <div class="view">
                <input type="checkbox" class="toggle" />
                <label>{"TODO 1"}</label>
                <button class="destroy" />
            </div>
            <input class="edit" type="text" value={"TODO 1"} hidden={true} />
        </li>
        <li class={"completed"}>
            <div class="view">
                <input type="checkbox" class="toggle" checked={true} />
                <label>{"TODO 2"}</label>
                <button class="destroy" />
            </div>
            <input class="edit" type="text" value={"TODO 2"} hidden={true} />
        </li>
        <li class={"editing"}>
            <div class="view">
                <input type="checkbox" class="toggle" />
                <label>{"TODO 3"}</label>
                <button class="destroy" />
            </div>
            <input class="edit" type="text" value={"TODO 3"} />
        </li>
    </ul>
    }
}
