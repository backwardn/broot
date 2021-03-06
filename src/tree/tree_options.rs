use {
    super::Sort,
    crate::pattern::*,
    clap::ArgMatches,
};

/// Options defining how the tree should be build and|or displayed
#[derive(Debug, Clone)]
pub struct TreeOptions {
    pub show_hidden: bool, // whether files whose name starts with a dot should be shown
    pub only_folders: bool, // whether to hide normal files and links
    pub show_counts: bool,  // whether to show the number of files (> 1 only for dirs)
    pub show_dates: bool,  // whether to show the last modified date
    pub show_sizes: bool,  // whether to show sizes of files and dirs
    pub show_git_file_info: bool,
    pub trim_root: bool,            // whether to cut out direct children of root
    pub show_permissions: bool,     // show classic rwx unix permissions (only on unix)
    pub respect_git_ignore: bool,   // hide files as requested by .gitignore ?
    pub filter_by_git_status: bool, // only show files whose git status is not nul
    pub pattern: InputPattern,           // an optional filtering/scoring pattern
    pub date_time_format: &'static str,
    pub sort: Sort,
}

impl TreeOptions {
    /// clone self but without the pattern (if any)
    pub fn without_pattern(&self) -> Self {
        TreeOptions {
            show_hidden: self.show_hidden,
            only_folders: self.only_folders,
            show_counts: self.show_counts,
            show_dates: self.show_dates,
            show_sizes: self.show_sizes,
            show_permissions: self.show_permissions,
            respect_git_ignore: self.respect_git_ignore,
            filter_by_git_status: self.filter_by_git_status,
            show_git_file_info: self.show_git_file_info,
            trim_root: self.trim_root,
            pattern: InputPattern::none(),
            date_time_format: self.date_time_format,
            sort: self.sort,
        }
    }
    /// counts must be computed, either for sorting or just for display
    pub fn needs_counts(&self) -> bool {
        self.show_counts || self.sort == Sort::Count
    }
    /// dates must be computed, either for sorting or just for display
    pub fn needs_dates(&self) -> bool {
        self.show_dates || self.sort == Sort::Date
    }
    /// sizes must be computed, either for sorting or just for display
    pub fn needs_sizes(&self) -> bool {
        self.show_sizes || self.sort == Sort::Size
    }
    pub fn needs_sum(&self) -> bool {
        self.needs_counts() || self.needs_dates() || self.needs_sizes()
    }
    /// this method does not exist, you saw nothing
    /// (at least don't call it other than with the config, once)
    pub fn set_date_time_format(&mut self, format: String) {
        self.date_time_format = Box::leak(format.into_boxed_str());
    }
    /// change tree options according to broot launch arguments
    pub fn apply(&mut self, cli_args: &ArgMatches<'_>) {
        if cli_args.is_present("sizes") {
            self.show_sizes = true;
        } else if cli_args.is_present("no-sizes") {
            self.show_sizes = false;
        }
        if cli_args.is_present("whale-spotting") {
            self.show_hidden = true;
            self.respect_git_ignore = false;
            self.sort = Sort::Size;
            self.show_sizes = true;
        }
        if cli_args.is_present("only-folders") {
            self.only_folders = true;
        } else if cli_args.is_present("no-only-folders") {
            self.only_folders = false;
        }
        if cli_args.is_present("hidden") {
            self.show_hidden = true;
        } else if cli_args.is_present("no-hidden") {
            self.show_hidden = false;
        }
        if cli_args.is_present("dates") {
            self.show_dates = true;
        } else if cli_args.is_present("no-dates") {
            self.show_dates = false;
        }
        if cli_args.is_present("permissions") {
            self.show_permissions = true;
        } else if cli_args.is_present("no-permissions") {
            self.show_permissions = false;
        }
        if cli_args.is_present("show-gitignored") {
            self.respect_git_ignore = false;
        } else if cli_args.is_present("no-show-gitignored") {
            self.respect_git_ignore = true;
        }
        if cli_args.is_present("show-git-info") {
            self.show_git_file_info = true;
        } else if cli_args.is_present("no-show-git-info") {
            self.show_git_file_info = false;
        }
        if cli_args.is_present("sort-by-count") {
            self.sort = Sort::Count;
            self.show_counts = true;
        }
        if cli_args.is_present("sort-by-date") {
            self.sort = Sort::Date;
            self.show_dates = true;
        }
        if cli_args.is_present("sort-by-size") {
            self.sort = Sort::Size;
            self.show_sizes = true;
        }
        if cli_args.is_present("no-sort") {
            self.sort = Sort::None;
        }
        if cli_args.is_present("trim-root") {
            self.trim_root = true;
        } else if cli_args.is_present("no-trim-root") {
            self.trim_root = false;
        }
    }
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            show_hidden: false,
            only_folders: false,
            show_counts: false,
            show_dates: false,
            show_sizes: false,
            show_git_file_info: false,
            trim_root: true,
            show_permissions: false,
            respect_git_ignore: true,
            filter_by_git_status: false,
            pattern: InputPattern::none(),
            date_time_format: "%Y/%m/%d %R",
            sort: Sort::None,
        }
    }
}
