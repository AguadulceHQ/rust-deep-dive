#[derive(Debug, PartialEq, Copy, Clone)]
enum ProjectPriority {
    High,
    Medium,
    Low,
}

struct Pipeline {
    projects: Vec<ProjectPriority>,
}

impl Pipeline {
    fn next_project(&self, priority: Option<ProjectPriority>) -> ProjectPriority {
        // we pass in an Option<ProjectPriority> and use a method on its value
        // unwrap_or_else is defined in std and it takes a closure without arguments that returns a value T (of the same type stored in the Some variant of the Option<T> in our case ProjectPriority
        // if the variant is None then the fn calls the closure and returns the value returned by the closure
        priority.unwrap_or_else(|| self.bottleneck())
    }

    fn bottleneck(&self) -> ProjectPriority {
        let mut num_low_priority = 0;
        let mut num_med_priority = 0;
        let mut num_high_priority = 0;

        for project in &self.projects {
            match project {
                ProjectPriority::Low => num_low_priority += 1,
                ProjectPriority::Medium => num_med_priority += 1,
                ProjectPriority::High => num_high_priority += 1,
            }
        }

        if num_low_priority > num_med_priority && num_low_priority > num_high_priority {
            ProjectPriority::Low
        } else if num_med_priority > num_high_priority {
            ProjectPriority::Medium
        } else {
            ProjectPriority::High
        }
    }
}

fn main() {
    let pipeline = Pipeline {
        projects: vec![
            ProjectPriority::Low,
            ProjectPriority::Medium,
            ProjectPriority::Medium,
            ProjectPriority::High,
        ],
    };

    let manager_priority = Some(ProjectPriority::High);
    let next_project = pipeline.next_project(manager_priority);
    println!(
        "The manager selected {:?} and the next project will be {:?}",
        manager_priority, next_project
    );

    // shadowing
    let manager_priority = None;
    let next_project = pipeline.next_project(manager_priority);
    println!(
        "The manager selected {:?} and the next project will be {:?}",
        manager_priority, next_project
    );
}
