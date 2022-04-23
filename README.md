# Possible `actix-web` memory leak?

This server uses `on_connect` to set a connection-wide state, with a struct called `Droppable`. This struct prints a line when it is getting dropped. For some reason, the struct does not get dropped when the connection completes. They only get dropped when the server is closing.
