//! Load your game assets with _type-safety_ and build loading screens that can
//! keep track of progress _consistently_.
//!
//! # Tasks
//! The generic [`Task`] struct represents a lazy loading operation that can
//! be combined and composed with other tasks. Most of the types representing
//! resources in Coffee have `load` functions that return a [`Task`].
//!
//! Tasks are defined declaratively in a functional style. This allows them to
//! keep track of all the work they have to complete before even executing them.
//! Read the [`Task`] docs to learn more!
//!
//! # Loading screens
//! The [`LoadingScreen`] trait allows you to implement a loading screen that is
//! compatible with any [`Task`]. Currently, Coffee includes a built-in loading
//! screen: [`ProgressBar`], which shows a simple progress bar with some text.
//!
//! [`Task`]: struct.Task.html
//! [`LoadingScreen`]: loading_screen/trait.LoadingScreen.html
//! [`ProgressBar`]: loading_screen/struct.ProgressBar.html
pub mod loading_screen;

pub use loading_screen::LoadingScreen;

use crate::graphics;

/// A `Task<T>` represents an operation that produces a value of type `T`.
///
/// # Laziness
/// A [`Task`] is just a recipe that describes how to produce a specific output,
/// like a function. They can be combined or transformed in certain ways and
/// run whenever needed.
///
/// Creating a [`Task`] consists in specifying this recipe. For instance,
/// we could define a task to load an `Image` like this:
///
/// ```
/// # use coffee::load::Task;
/// # use coffee::graphics::Image;
/// #
/// let load_image = Task::using_gpu(|gpu| Image::new(gpu, "my-image.png"));
/// ```
///
/// Here we just _describe_ how to load an image, we do not load it right away.
/// This is how [`Image::load`] works, you should use that instead of writing
/// this over and over!
///
/// # Composition
/// Tasks can be combined easily thanks to the [`Join`] trait.
///
/// Let's say we have a `Task<Image>` and a `Task<TextureArray>`, we can easily
/// obtain a `Task<(Image, TextureArray)>` like this:
///
/// ```
/// # use coffee::load::Task;
/// # let load_image = Task::new(|| ());
/// # let load_texture_array = Task::new(|| ());
/// #
/// use coffee::load::Join;
///
/// let combined_task = (load_image, load_texture_array).join();
/// ```
///
/// You can do this for up to 8 tasks. However, consider grouping task output in
/// meaningful structs using [`map`]:
///
/// ```
/// # use coffee::load::Task;
/// # use coffee::graphics::Image;
/// #
/// use coffee::load::Join;
///
/// pub struct PlayerAssets {
///     idle: Image,
///     running: Image,
/// }
///
/// impl PlayerAssets {
///     pub fn load() -> Task<PlayerAssets> {
///         (
///             Image::load("player/idle.png"),
///             Image::load("player/running.png"),
///         )
///             .join()
///             .map(|(idle, running)| PlayerAssets { idle, running })
///     }
/// }
/// ```
///
/// [`Task`]: struct.Task.html
/// [`Join`]: trait.Join.html
/// [`Image::load`]: ../graphics/struct.Image.html#method.load
/// [`map`]: #method.map
pub struct Task<T> {
    total_work: u32,
    function: Box<Fn(&mut Worker) -> T>,
}

impl<T> Task<T> {
    /// Create a new task from a lazy operation.
    ///
    /// Imagine we had to generate a random game map, we could represent this
    /// as a `Task`:
    /// ```
    /// # use coffee::load::Task;
    /// struct Map {
    ///     // ...
    /// }
    ///
    /// impl Map {
    ///     pub fn generate() -> Map {
    ///         Map { /*...*/ }
    ///     }
    /// }
    ///
    /// let generate_map = Task::new(Map::generate);
    /// ```
    ///
    /// [`Task`]: struct.Task.html
    pub fn new<F>(f: F) -> Task<T>
    where
        F: 'static + Fn() -> T,
    {
        Task {
            total_work: 1,
            function: Box::new(move |_| f()),
        }
    }

    /// Create a new task that uses the GPU.
    ///
    /// You can use this to load and prepare graphical assets.
    ///
    /// Keep in mind that many types in [`graphics`] already implement loading
    /// methods returning a `Task` (like [`Image::load`] or [`Font::load`]).
    /// Before using this, check out whether whatever you want to load has
    /// already a useful helper that suits your needs!
    ///
    /// [`graphics`]: ../graphics/index.html
    /// [`Task`]: struct.Task.html
    /// [`Image::load`]: ../graphics/struct.Image.html#method.load
    /// [`Font::load`]: ../graphics/struct.Font.html#method.load
    pub fn using_gpu<F>(f: F) -> Task<T>
    where
        F: 'static + Fn(&mut graphics::Gpu) -> T,
    {
        Task::sequence(1, move |worker| {
            let result = f(worker.gpu());

            worker.notify_progress(1);

            result
        })
    }

    pub(crate) fn sequence<F>(total_work: u32, f: F) -> Task<T>
    where
        F: 'static + Fn(&mut Worker) -> T,
    {
        Task {
            total_work,
            function: Box::new(f),
        }
    }

    /// Add a title to a task.
    ///
    /// The title will be used when reporting progress once the task is run.
    /// This allows task runners, like loading screens, to show additional
    /// feedback to the user.
    ///
    /// For example, let's say we want to generate a map and load some terrain
    /// assets. We can define a couple stages for each task:
    /// ```
    /// # use coffee::load::Task;
    /// # use coffee::graphics::Image;
    /// # struct Map;
    /// # impl Map {
    /// # fn generate() -> Map { Map }
    /// # }
    /// # struct TerrainAssets;
    /// # impl TerrainAssets {
    /// # fn load() -> Task<()> { Task::new(|| ()) }
    /// # }
    /// use coffee::load::Join;
    ///
    /// let load_game =
    ///     (
    ///         Task::stage("Generating map...", Task::new(Map::generate)),
    ///         Task::stage("Loading terrain...", TerrainAssets::load())
    ///     )
    ///         .join();
    /// ```
    /// If we then used this task with the [`ProgressBar`] loading screen, it
    /// would show each of these titles on top of the progress bar when their
    /// according tasks are being run.
    ///
    /// [`ProgressBar`]: loading_screen/struct.ProgressBar.html
    pub fn stage<S: Into<String>>(title: S, task: Task<T>) -> Task<T>
    where
        T: 'static,
    {
        let title = title.into();

        Task {
            total_work: task.total_work,
            function: Box::new(move |worker| {
                worker.with_stage(title.clone(), &task.function)
            }),
        }
    }

    /// Get the total units of work of the task.
    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    /// Transform the output of a task.
    ///
    /// As [explained above], use this method to make your tasks return your
    /// own custom types, enhancing composability.
    ///
    /// [explained above]: #composition
    pub fn map<F, A>(self, f: F) -> Task<A>
    where
        T: 'static,
        F: 'static + Fn(T) -> A,
    {
        Task {
            total_work: self.total_work,
            function: Box::new(move |worker| f((self.function)(worker))),
        }
    }

    /// Run a task and obtain the produced value.
    ///
    /// You can provide a function to keep track of [`Progress`].
    ///
    /// As of now, this method needs a [`Window`] because tasks are mostly
    /// meant to be used with loading screens. However, the `Task` abstraction
    /// is generic enough to be useful in other scenarios and we could work on
    /// removing this dependency. If you have a particular use case for them,
    /// feel free to [open an issue] detailing it!
    ///
    /// [`Progress`]: struct.Progress.html
    /// [`Window`]: ../graphics/window/struct.Window.html
    /// [open an issue]: https://github.com/hecrj/coffee/issues
    pub fn run<F>(self, window: &mut graphics::Window, mut on_progress: F) -> T
    where
        F: FnMut(&Progress, &mut graphics::Window) -> (),
    {
        let mut worker = Worker {
            window,
            listener: &mut on_progress,
            progress: Progress {
                total_work: self.total_work,
                work_completed: 0,
                stages: Vec::new(),
            },
        };

        worker.notify_progress(0);

        (self.function)(&mut worker)
    }
}

pub(crate) struct Worker<'a> {
    window: &'a mut graphics::Window,
    listener: &'a mut FnMut(&Progress, &mut graphics::Window) -> (),
    progress: Progress,
}

impl<'a> Worker<'a> {
    pub fn gpu(&mut self) -> &mut graphics::Gpu {
        self.window.gpu()
    }

    pub fn notify_progress(&mut self, work: u32) {
        self.progress.work_completed += work;

        (self.listener)(&self.progress, self.window);
    }

    pub fn with_stage<T>(
        &mut self,
        title: String,
        f: &Box<Fn(&mut Worker) -> T>,
    ) -> T {
        self.progress.stages.push(title);
        self.notify_progress(0);

        let result = f(self);
        let _ = self.progress.stages.pop();

        result
    }
}

/// The progress of a task.
pub struct Progress {
    total_work: u32,
    work_completed: u32,
    stages: Vec<String>,
}

impl Progress {
    /// Get the total amount of work of the related task for this progress.
    pub fn total_work(&self) -> u32 {
        self.total_work
    }

    /// Get the amount of completed work.
    ///
    /// The returned value is guaranteed to be in [0, total_work].
    pub fn completed_work(&self) -> u32 {
        self.work_completed.min(self.total_work)
    }

    /// Get the amount of progress as a percentage.
    ///
    /// You can use this value directly in your loading screen.
    pub fn percentage(&self) -> f32 {
        (self.completed_work() as f32 / self.total_work.max(1) as f32 * 100.0)
    }

    /// Get the title of the current stage, if there is one.
    ///
    /// You can use this to provide additional feedback to users.
    pub fn stage(&self) -> Option<&String> {
        self.stages.last()
    }
}

/// Join tasks with ease.
///
/// Learn more about how to use this trait in the [`Task`] docs.
///
/// [`Task`]: struct.Task.html#composition
pub trait Join {
    type Type;

    fn join(self) -> Task<Self::Type>;
}

impl<A: 'static, B: 'static> Join for (Task<A>, Task<B>) {
    type Type = (A, B);

    fn join(self) -> Task<(A, B)> {
        let (loader_a, loader_b) = self;

        Task::sequence(
            loader_a.total_work() + loader_b.total_work(),
            move |task| ((loader_a.function)(task), (loader_b.function)(task)),
        )
    }
}

impl<A: 'static, B: 'static, C: 'static> Join for (Task<A>, Task<B>, Task<C>) {
    type Type = (A, B, C);

    fn join(self) -> Task<(A, B, C)> {
        let (loader_a, loader_b, loader_c) = self;

        ((loader_a, loader_b).join(), loader_c)
            .join()
            .map(|((a, b), c)| (a, b, c))
    }
}
