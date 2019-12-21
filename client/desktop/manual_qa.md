MINIMAL WORKFLOW

Preliminary
	a. Make an account
	b. Check title/timestamp/avatar label on Note to Self

Messages
	a. Send messages, check timestamps in sidebar and on bubbles, try deletion (check this updates sidebar timestamp)
	b. Send a reply, delete the message it was replying to
	c. Send an attachment (for now, only images)
	d. Check that clear history in options menu in chat bar works (and also updates sidebar)

Message view controls
	a. check that up and down arrow keys scroll, page up and page down, home and end
	b. Check that j/k scroll up and down, g and G take you to beginning and end

Contact flow
	a. Add a contact
	b. Messages (a - d)

Group flow
	a. Start group flow, try to set a group picture, clear group picture, set again
	b. Check that title and picture after creating group are correct (and that avatar is square)
	c. Messages (a - d)

Config options
	a. Check that config menu opens both with clicking on own avatar and with toolbar
	b. Change profile picture, display name
	
Global search
	a. Search conversations and messages in search menu in sidebar, check that matches are correct

---

ADDITIONAL CHECKS

Messages (additional)
	d. Check flurry logic - messages sent in a row should only have an avatar on the first
	e. Check colors/timestamps on reply bubbles, make sure they match originals
	f. Check that setting disappearing timer to off removes timer icon on ensuing message bubbles

Inline conversation search
	a. Match on multiple messages, check that up/down arrows and enter key page through matches (indices should loop properly)
	b. Check highlighting
	c. Search something that will match on one message; delete that message, check that search clears indices
	d. Search and match on multiple messages, and try the following:
		- Deleting a message from the middle of matches
		- Deleting the message at the end of matches
		- Deleting the message at the beginning
	e. Match on multiple and try deleting messages before and after the cursor

Chat text area
	a. Start a reply, close out the reply, send a message, make sure it is no longer a reply
	b. Type a long message, make sure text area clips and scrolls correctly

Resize behavior
	a. Move split handle, check that resizing sidebar and chatview does not move buttons offscreen
	b. Check that message bubbles rewrap correctly on chatview resize
	c. Check that header text and conversation labels elide on resize 
	
Buttons and menus
	a. Check that each button in header bars opens correct view/menu
	b. Check that changing disappearing timer updates other end
	c. Check that all menus are native

Errors
	a. Check that error popups appear (e.g., try adding a nonexistent contact, reopen app)
	
