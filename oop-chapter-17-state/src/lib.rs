// Why didn't we use an enum for the different possible post states?
// While viable, that solution would cause a lot of match expression
// checks throughout the code, which could get more repetitive than
// the current trait object solution below.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // ""
        // want the value returned to depend on the current state
        // of the Post, so going to have Post delegate to a
        // content method defined on its state
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            // call the internal request_review
            // the second request_review method consumes the current
            // state and returns a new state
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // This syntax takes ownership of Box<Self>, invalidating the
    // old state so the state value of the Post can transform
    // into a new state.  To consume the old state, the
    // request_review method needs to take ownership of the state
    // value.  This is where Option in the state field comes in:
    // We call the take method to take the Some value out of the
    // state field and leave a None in its place, because Rust
    // doesn't let us have unpopulated fields in structs.  This
    // lets us move the state value out of a Post rather than just
    // borrowing it.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // add content default implementation, which allows us not to
    // need implementations of content on the Draft and PendingReview
    // sructs
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // calling approve on a draft will have no effect because
    // approve will return self
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // when we call approve on a PendingReivew, it returns a
    // new, boxed instance of the Published struct
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // returns self because want the post to stay in the Published
    // state when request_review is called on it
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // returns self because want the post to stay in the Published
    // state when approve is called on it
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
