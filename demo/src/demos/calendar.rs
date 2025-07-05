use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn CalendarDemo() -> impl IntoView {
    let (selected_date, set_selected_date) = signal(None::<String>);

    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Calendar"</h1>
        //     <p class="text-base-content/70">
        //         "Calendar component for date selection and scheduling"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Calendar"</h2>
        //         <Calendar selected_date=selected_date class="bg-base-100 shadow-lg rounded-lg" />

        //         <h2 class="text-xl font-semibable">"Calendar with Header"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <div class="flex items-center justify-between mb-4">
        //                     <h3 class="card-title">"December 2024"</h3>
        //                     <div class="flex gap-2">
        //                         <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                             <Icon icon=icondata::AiLeftOutlined />
        //                         </Button>
        //                         <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                             <Icon icon=icondata::AiRightOutlined />
        //                         </Button>
        //                     </div>
        //                 </div>
        //                 <Calendar
        //                     selected_date=selected_date
        //                     on:date_select=move |date: String| {
        //                         set_selected_date(Some(date));
        //                     }
        //                 />
        //                 <div class="mt-4">
        //                     <p class="text-sm">
        //                         "Selected date: "
        //                         <span class="font-semibold">
        //                             {move || selected_date().unwrap_or_else(|| "None".to_string())}
        //                         </span>
        //                     </p>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Event Calendar"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Team Schedule"</h3>
        //                 <div class="grid grid-cols-7 gap-1 text-center text-sm mb-2">
        //                     <div class="font-semibold p-2">"Sun"</div>
        //                     <div class="font-semibold p-2">"Mon"</div>
        //                     <div class="font-semibold p-2">"Tue"</div>
        //                     <div class="font-semibold p-2">"Wed"</div>
        //                     <div class="font-semibold p-2">"Thu"</div>
        //                     <div class="font-semibold p-2">"Fri"</div>
        //                     <div class="font-semibold p-2">"Sat"</div>
        //                 </div>
        //                 <div class="grid grid-cols-7 gap-1">
        //                     // Empty cells for previous month
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>

        //                     // December 1 - 7
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"1"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"2"</div>
        //                         <div class="text-xs bg-primary text-primary-content rounded px-1 mt-1">
        //                             "Meeting"
        //                         </div>
        //                     </div>

        //                     // December 8 - 14
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"8"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"9"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"10"</div>
        //                         <div class="text-xs bg-success text-success-content rounded px-1 mt-1">
        //                             "Launch"
        //                         </div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"11"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"12"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"13"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"14"</div>
        //                     </div>

        //                     // December 15 - 21
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"15"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"16"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"17"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"18"</div>
        //                         <div class="text-xs bg-warning text-warning-content rounded px-1 mt-1">
        //                             "Review"
        //                         </div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"19"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"20"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"21"</div>
        //                     </div>

        //                     // December 22 - 28
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"22"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"23"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center font-semibold text-error">"24"</div>
        //                         <div class="text-xs bg-error text-error-content rounded px-1 mt-1">
        //                             "Holiday"
        //                         </div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center font-semibold text-error">"25"</div>
        //                         <div class="text-xs bg-error text-error-content rounded px-1 mt-1">
        //                             "Christmas"
        //                         </div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"26"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"27"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"28"</div>
        //                     </div>

        //                     // December 29 - 31
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"29"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center">"30"</div>
        //                     </div>
        //                     <div class="aspect-square p-1 hover:bg-base-200 rounded cursor-pointer">
        //                         <div class="text-center font-semibold text-error">"31"</div>
        //                         <div class="text-xs bg-accent text-accent-content rounded px-1 mt-1">
        //                             "NYE"
        //                         </div>
        //                     </div>
        //                     // Fill remaining cells
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                     <div class="p-2"></div>
        //                 </div>

        //                 <div class="mt-4">
        //                     <h4 class="font-semibold mb-2">"Legend"</h4>
        //                     <div class="flex flex-wrap gap-2 text-xs">
        //                         <Badge color=BadgeColor::Primary>"Meeting"</Badge>
        //                         <Badge color=BadgeColor::Success>"Launch"</Badge>
        //                         <Badge color=BadgeColor::Warning>"Review"</Badge>
        //                         <Badge color=BadgeColor::Error>"Holiday"</Badge>
        //                         <Badge color=BadgeColor::Accent>"Special"</Badge>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Compact Calendar"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg">"Mini Calendar"</h3>
        //                     <div class="grid grid-cols-7 gap-px text-xs text-center bg-base-300 rounded">
        //                         <div class="bg-base-100 p-1 font-semibold">"S"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"M"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"T"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"W"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"T"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"F"</div>
        //                         <div class="bg-base-100 p-1 font-semibold">"S"</div>

        //                         // Calendar days (simplified)
        //                         <div class="bg-base-100 p-1 opacity-50"></div>
        //                         <div class="bg-base-100 p-1">"1"</div>
        //                         <div class="bg-base-100 p-1">"2"</div>
        //                         <div class="bg-base-100 p-1">"3"</div>
        //                         <div class="bg-base-100 p-1">"4"</div>
        //                         <div class="bg-base-100 p-1">"5"</div>
        //                         <div class="bg-base-100 p-1">"6"</div>

        //                         <div class="bg-base-100 p-1">"7"</div>
        //                         <div class="bg-base-100 p-1">"8"</div>
        //                         <div class="bg-base-100 p-1">"9"</div>
        //                         <div class="bg-base-100 p-1">"10"</div>
        //                         <div class="bg-base-100 p-1">"11"</div>
        //                         <div class="bg-base-100 p-1">"12"</div>
        //                         <div class="bg-base-100 p-1">"13"</div>

        //                         <div class="bg-base-100 p-1">"14"</div>
        //                         <div class="bg-primary text-primary-content p-1 font-semibold">"15"</div>
        //                         <div class="bg-base-100 p-1">"16"</div>
        //                         <div class="bg-base-100 p-1">"17"</div>
        //                         <div class="bg-base-100 p-1">"18"</div>
        //                         <div class="bg-base-100 p-1">"19"</div>
        //                         <div class="bg-base-100 p-1">"20"</div>

        //                         <div class="bg-base-100 p-1">"21"</div>
        //                         <div class="bg-base-100 p-1">"22"</div>
        //                         <div class="bg-base-100 p-1">"23"</div>
        //                         <div class="bg-base-100 p-1">"24"</div>
        //                         <div class="bg-base-100 p-1">"25"</div>
        //                         <div class="bg-base-100 p-1">"26"</div>
        //                         <div class="bg-base-100 p-1">"27"</div>

        //                         <div class="bg-base-100 p-1">"28"</div>
        //                         <div class="bg-base-100 p-1">"29"</div>
        //                         <div class="bg-base-100 p-1">"30"</div>
        //                         <div class="bg-base-100 p-1">"31"</div>
        //                         <div class="bg-base-100 p-1 opacity-50">"1"</div>
        //                         <div class="bg-base-100 p-1 opacity-50">"2"</div>
        //                         <div class="bg-base-100 p-1 opacity-50">"3"</div>
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg">"Today's Schedule"</h3>
        //                     <div class="space-y-2 text-sm">
        //                         <div class="flex items-center gap-2">
        //                             <div class="w-2 h-2 bg-primary rounded-full"></div>
        //                             <span class="font-medium">"9:00 AM"</span>
        //                             <span>"Team Standup"</span>
        //                         </div>
        //                         <div class="flex items-center gap-2">
        //                             <div class="w-2 h-2 bg-success rounded-full"></div>
        //                             <span class="font-medium">"11:00 AM"</span>
        //                             <span>"Client Presentation"</span>
        //                         </div>
        //                         <div class="flex items-center gap-2">
        //                             <div class="w-2 h-2 bg-warning rounded-full"></div>
        //                             <span class="font-medium">"2:00 PM"</span>
        //                             <span>"Project Review"</span>
        //                         </div>
        //                         <div class="flex items-center gap-2">
        //                             <div class="w-2 h-2 bg-info rounded-full"></div>
        //                             <span class="font-medium">"4:00 PM"</span>
        //                             <span>"Documentation"</span>
        //                         </div>
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Date Range Picker"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title">"Select Date Range"</h3>
        //                 <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        //                     <div>
        //                         <h4 class="font-semibold mb-2">"Start Date"</h4>
        //                         <Calendar selected_date=selected_date />
        //                     </div>
        //                     <div>
        //                         <h4 class="font-semibold mb-2">"End Date"</h4>
        //                         <Calendar selected_date=selected_date />
        //                     </div>
        //                 </div>
        //                 <div class="mt-4 p-4 bg-base-200 rounded-lg">
        //                     <h5 class="font-semibold mb-2">"Selected Range"</h5>
        //                     <p class="text-sm">
        //                         "From: Dec 15, 2024 to Dec 20, 2024 (5 days)"
        //                     </p>
        //                     <div class="mt-3 flex gap-2">
        //                         <Button color=ButtonColor::Primary size=ButtonSize::Sm>
        //                             "Apply"
        //                         </Button>
        //                         <Button style=ButtonStyle::Ghost size=ButtonSize::Sm>
        //                             "Clear"
        //                         </Button>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Calendar with Time Slots"</h2>
        //         <div class="grid grid-cols-1 xl:grid-cols-3 gap-6">
        //             <div class="xl:col-span-2">
        //                 <Card class="bg-base-100 shadow-xl">
        //                     <CardBody>
        //                         <h3 class="card-title">"Appointment Scheduler"</h3>
        //                         <Calendar selected_date=selected_date />
        //                     </CardBody>
        //                 </Card>
        //             </div>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title text-lg">"Available Times"</h3>
        //                     <p class="text-sm opacity-70 mb-4">"December 15, 2024"</p>
        //                     <div class="space-y-2">
        //                         <Button
        //                             style=ButtonStyle::Outline
        //                             size=ButtonSize::Sm
        //                             class="w-full justify-start"
        //                         >
        //                             "9:00 AM"
        //                         </Button>
        //                         <Button
        //                             color=ButtonColor::Primary
        //                             size=ButtonSize::Sm
        //                             class="w-full justify-start"
        //                         >
        //                             "10:30 AM"
        //                         </Button>
        //                         <Button
        //                             style=ButtonStyle::Outline
        //                             size=ButtonSize::Sm
        //                             class="w-full justify-start"
        //                         >
        //                             "2:00 PM"
        //                         </Button>
        //                         <Button
        //                             style=ButtonStyle::Outline
        //                             size=ButtonSize::Sm
        //                             class="w-full justify-start"
        //                         >
        //                             "3:30 PM"
        //                         </Button>
        //                         <Button
        //                             disabled=true
        //                             size=ButtonSize::Sm
        //                             class="w-full justify-start"
        //                         >
        //                             "4:00 PM (Booked)"
        //                         </Button>
        //                     </div>

        //                     <div class="mt-6">
        //                         <h4 class="font-semibold mb-2">"Selected Appointment"</h4>
        //                         <div class="bg-base-200 p-3 rounded-lg text-sm">
        //                             <p class="font-medium">"December 15, 2024"</p>
        //                             <p>"10:30 AM - 11:30 AM"</p>
        //                             <p class="text-xs opacity-70 mt-1">"1 hour session"</p>
        //                         </div>
        //                         <Button color=ButtonColor::Success class="w-full mt-3">
        //                             "Book Appointment"
        //                         </Button>
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>
        //     </div>
        // </div>
    }
}
