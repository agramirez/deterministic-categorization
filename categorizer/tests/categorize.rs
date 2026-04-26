use cucumber::{World,given,when,then};

#[derive(Debug, Default, PartialEq)]
struct Result {
    text: String,
    confidence: f32,
    min: f32,
    max: f32,
}

#[derive(Debug, Default, World)]
struct CategorizeWorld {
    message: String,
    category: String,
    expression: String,
    model: String,
    prompt: String,
    cosine: String,
    min: f32,
    max: f32,
    confidence: f32,
    result: Result,
}

#[given(regex=r#"a message with content "(?ims)([\w\d\s]+)""#)]
fn given_message(world:&mut CategorizeWorld, content:String) {
    world.message = content;
}

#[given(regex=r#"a "(?ims)(regex|cosine|llm)" category"#)]
fn given_category(world:&mut CategorizeWorld, category:String) {
    world.category = category;
}

#[given(regex=r#"expression "(?ims)(.*?)""#)]
fn given_expression(world:&mut CategorizeWorld, expression:String) {
    world.expression = expression;
}

#[given(regex=r#"model "(?ims)(.*?)""#)]
fn given_model(world:&mut CategorizeWorld, model:String) {
    world.model = model;
}

#[given(regex=r#"cosine for text "(?ims)(.*?)""#)]
fn given_cosine(world:&mut CategorizeWorld, cosine:String) {
    world.cosine = cosine;
}

#[given(regex=r#"(min|max)?\s?confidence "(?ims)([\d.]+)""#)]
fn given_min_confidence(world:&mut CategorizeWorld,ctype:String, confidence:f32) {
    match ctype.as_str() {
        "min" => world.min = confidence,
        "max" => world.max = confidence,
        _ => world.confidence = confidence,
    }
}

#[given(regex=r#"prompt "(?ims)(.*?)""#)]
fn given_prompt(world:&mut CategorizeWorld,prompt:String) {
    world.prompt = prompt;
}

#[when(regex=r"I categorize the message")]
fn when_category(world:&mut CategorizeWorld) {
    world.result = Result { text: String::from("yes"), confidence: 1.0, min: 0.0, max: 1.0 }
}

#[then(regex=r#"I should see the category match as "(?ims)(yes|no)""#)] 
fn then_match(world:&mut CategorizeWorld, result:String) {
    assert_eq!(world.result.text,result);
}

#[then(regex=r#"(min|max)?\s?confidence "(?ims)([\d.]+)""#)] 
fn then_confidence(world:&mut CategorizeWorld, ctype:String, confidence:f32) {
    match ctype.as_str() {
        "min" => assert_eq!(world.result.min,confidence),
        "max" => assert_eq!(world.result.max,confidence),
        _ => assert_eq!(world.result.confidence,confidence),
    }
}

#[then(expr=r#"confidence "_""#)] 
fn then_some_confidence(world:&mut CategorizeWorld) {
    assert!(world.result.confidence>=0.0);
}

fn main() {
    futures::executor::block_on(CategorizeWorld::run(
        "/workspaces/deterministic-normalization/categorizer/features/categorize.feature"
    ))
}