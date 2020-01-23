## ChatBubble Architecture

chatbubbles are very content rich and polymorphic, as such they are the most complex component of this project. This readme is intended to help developers navigate this component.

### Entry Points
---

#### DefaultBubble.qml
this is the main entry point for most chat bubbles. all bindings should be described with a short comment. DefaultBubble primarily contains a series of loaders that run a switch case to select which of the multidtudinous subcomponents to use to populate the main bubble content. 

There is additionally a near exact copy of DefaultBubble.qml called ChatBubble.qml, this is used in the message information view to display copy of the message without any mouse even bindings.



