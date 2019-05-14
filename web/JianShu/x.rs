//rust--状态模式

//源码: state1.rs
#[derive(Debug)]
struct StateMachine<T> {
    state: T
}


#[derive(Debug)]
struct AmState<'a> {
    name: &'a str,
    stations: Vec<&'a str>,
    pos: i32
}


#[derive(Debug)]
struct FmState<'a> {
    name: &'a str,
    stations: Vec<&'a str>,
    pos: i32
}


impl<'a> StateMachine<AmState<'a>> {
    fn new() -> Self {
        StateMachine {
            state: AmState {
                name: "AmState",
                stations: vec!["1250", "1380", "1510"],
                pos: 0
            }
        }
    }
}


impl<'a> From<StateMachine<AmState<'a>>> for StateMachine<FmState<'a>> {
    fn from(_: StateMachine<AmState>) -> StateMachine<FmState> {
        StateMachine {
            state: FmState {
                name: "FmState",
                stations: vec!["81.3", "89.1", "103.9"],
                pos: 0
            }
        }
    }
}


impl<'a> From<StateMachine<FmState<'a>>> for StateMachine<AmState<'a>> {
    fn from(_: StateMachine<FmState>) -> StateMachine<AmState> {
        StateMachine {
            state: AmState {
                name: "AmState",
                stations: vec!["1250", "1380", "1510"],
                pos: 0
            }
        }
    }
}


#[derive(Debug)]
enum StateMachineWrapper<'a> {
    AmState(StateMachine<AmState<'a>>),
    FmState(StateMachine<FmState<'a>>)
}


impl<'a> StateMachineWrapper<'a> {
    fn step(mut self) -> Self {
        self = match self {
            StateMachineWrapper::AmState(val) => StateMachineWrapper::FmState(val.into()),
            StateMachineWrapper::FmState(val) => StateMachineWrapper::AmState(val.into())
        };
        self
    }
}


// TODO: 这里只实现了AM和FM的切换, 如何便捷的实现电台的切换?
fn main() {
    let mut state = StateMachineWrapper::AmState(StateMachine::new());
    println!("state: {:?}", state);
    state = state.step();
    println!("state: {:?}", state);
}

//源码: state2.rs
// 这种写法比较直, 也就是说任何环节发生变化,
// 都需要全部重新定义.

struct Post {
    content: String
}

struct DraftPost {
    content: String
}

struct PendingPost {
    content: String,
    approvals: u32

}

impl Post {
    fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn req_review(self) -> PendingPost {
        PendingPost {
            content: self.content,
            approvals: 0
        }
    }

    fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
    }
}

enum PublishResult {
    PendingPost(PendingPost),
    Post(Post)
}

impl PendingPost {
    fn approve(&mut self) {
        self.approvals += 1;
    }
    fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
    fn publish(self) -> PublishResult {
        if self.approvals > 1 {
            PublishResult::Post(Post{content: self.content})
        } else {
            PublishResult::PendingPost(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let mut pending = draft.req_review();
        pending.approve();
        pending.approve();
        let publish = pending.publish();
        match publish {
            PublishResult::Post(p) => assert_eq!(p.content(),
                                                 "ashish first post"),
            _ => assert!(false)
        }
    }

    #[test]
    fn reject_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let pending = draft.req_review();
        let mut again_draft = pending.reject();
        again_draft.add_text(".. after first one..");
    }

    #[test]
    fn two_approvals_workflow() {
        let mut draft = Post::new();
        draft.add_text("ashish first post");
        let mut pending = draft.req_review();
        pending.approve();
        match pending.publish() {
            PublishResult::PendingPost(_) => assert!(true),
            _ => assert!(false)
        }
    }
}

//源码: state3.rs
// 这种写法稍微灵活一些, 但也更复杂.
// 但仍然不够灵活.
// 参考: https://hoverbear.org/2016/10/12/rust-state-machine-pattern/

// This is our state machine.
#[derive(Debug)]
struct BottleFillingMachine<S> {
    shared_value: usize,
    state: S
}


// The following states can be the 'S' in StateMachine<S>
#[derive(Debug)]
struct Waiting {
    waiting_time: std::time::Duration,
}


#[derive(Debug)]
struct Filling {
    rate: usize,
}


#[derive(Debug)]
struct Done;


// Our Machine starts in the 'Waiting' state.
impl BottleFillingMachine<Waiting> {
    fn new(shared_value: usize) -> Self {
        BottleFillingMachine {
            shared_value: shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            }
        }
    }
}


// The following are the defined transitions between states.
impl From<BottleFillingMachine<Waiting>> for BottleFillingMachine<Filling> {
    fn from(val: BottleFillingMachine<Waiting>) -> BottleFillingMachine<Filling> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Filling {
                rate: 1,
            }
        }
    }
}

impl From<BottleFillingMachine<Filling>> for BottleFillingMachine<Done> {
    fn from(val: BottleFillingMachine<Filling>) -> BottleFillingMachine<Done> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Done,
        }
    }
}


impl From<BottleFillingMachine<Done>> for BottleFillingMachine<Waiting> {
    fn from(val: BottleFillingMachine<Done>) -> BottleFillingMachine<Waiting> {
        BottleFillingMachine {
            shared_value: val.shared_value,
            state: Waiting {
                waiting_time: std::time::Duration::new(0, 0),
            }
        }
    }
}


// Here is we're building an enum so we can contain this state machine in a parent.
#[derive(Debug)]
enum BottleFillingMachineWrapper {
    Waiting(BottleFillingMachine<Waiting>),
    Filling(BottleFillingMachine<Filling>),
    Done(BottleFillingMachine<Done>),
}


// 这里使用的是mut self, 表示它是一个实例化对象.
// 实例化对象证明enum BottleFillingMachineWrapper的成员存在有效对象.
// 因此BottleFillingMachineWrapper::Waiting(val) 中的 val 是一个结构体.
// 又由于val这个具体是一个struct且实现了From trait, 因此使用val.into会得到这个具体值.
// 最终将enum的某个状态转移到另外一个状态并且保证值的一致性.
// 总结: 这个就是状态机核心流转机制.
impl BottleFillingMachineWrapper {
    fn step(mut self) -> Self {
        self = match self {
            BottleFillingMachineWrapper::Waiting(val) => BottleFillingMachineWrapper::Filling(val.into()),
            BottleFillingMachineWrapper::Filling(val) => BottleFillingMachineWrapper::Done(val.into()),
            BottleFillingMachineWrapper::Done(val) => BottleFillingMachineWrapper::Waiting(val.into()),
        };
        self
    }
}


// The structure with a parent.
// 工厂模式: 帮你初始化和实例化状态机枚举对象: BottleFillingMachineWrapper::Wait(x)
//           帮你初始化和实例化状态机对象: BottleFillingMachine::new(0)
#[derive(Debug)]
struct Factory {
    bottle_filling_machine: BottleFillingMachineWrapper,
}


impl Factory {
    fn new() -> Self {
        Factory {
            bottle_filling_machine: BottleFillingMachineWrapper::Waiting(BottleFillingMachine::new(0)),
        }
    }
}


// 使用工厂模式作为开端, 然后运行step() 获得一个状态.
fn main() {
    // 得到Waiting状态
    let mut the_factory = Factory::new();
    println!("the_factory.bottle_filling_machine.step(): {:?}", the_factory);

    // 得到Filling状态
    the_factory.bottle_filling_machine = the_factory.bottle_filling_machine.step();
    println!("the_factory.bottle_filling_machine.step(): {:?}", the_factory);

    // 得到Done状态
    the_factory.bottle_filling_machine = the_factory.bottle_filling_machine.step();
    println!("the_factory.bottle_filling_machine.step(): {:?}", the_factory);

    // 得到Waiting状态
    the_factory.bottle_filling_machine = the_factory.bottle_filling_machine.step();
    println!("the_factory.bottle_filling_machine.step(): {:?}", the_factory);
}