// use std::collections::HashMap;

// struct FullScript {
//     methods: HashMap<String, MethodItemBody>,
// }

// fn execute(&methodItemBody: MethodItemBody, &args: Vec<Object>, &script FullScript) {

//     //results of othe methods
//     let mut tasksWithKeys: HashMap<String, task>; //createTasks

//     let mut isEverythingExecuted = false;
//     while !isEverythingExecuted {
//         // let variables Vec<String>  //element is like tasksWithKeys[line.args[0]]
//         for line in methodItemBody.lines {
//             //if all argument is ready
//             let resultsOfRequiredTasks: Vec<Object>;
//             tasksWithKeys[line.variableName] = execute(
//                 &script.methods[line.methodName], &resultsOfRequiredTasks, &script);
//         }
//     }
// }

// struct MethodItemBody {
//     lines: Vec<methodLine>,
// }

// struct methodLine {
//     variableName: String,
//     methodName: String, //methodItemBodyKey
//     args: Vec<String>,
// }
