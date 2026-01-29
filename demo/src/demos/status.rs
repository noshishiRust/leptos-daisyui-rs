use leptos::prelude::*;

#[component]
pub fn StatusDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"Status"</h1>
        //     <p class="text-base-content/70">
        //         "Status components to show different states and conditions"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic Status"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Status color=StatusColor::Success>
        //                 "Online"
        //             </Status>
        //             <Status color=StatusColor::Warning>
        //                 "Away"
        //             </Status>
        //             <Status color=StatusColor::Error>
        //                 "Offline"
        //             </Status>
        //             <Status color=StatusColor::Info>
        //                 "Idle"
        //             </Status>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Status with Icons"</h2>
        //         <div class="flex flex-wrap gap-4">
        //             <Status color=StatusColor::Success>
        //                 <Icon icon=icondata::AiCheckCircleOutlined class="mr-2" />
        //                 "Completed"
        //             </Status>
        //             <Status color=StatusColor::Warning>
        //                 <Icon icon=icondata::AiClockCircleOutlined class="mr-2" />
        //                 "In Progress"
        //             </Status>
        //             <Status color=StatusColor::Error>
        //                 <Icon icon=icondata::AiCloseCircleOutlined class="mr-2" />
        //                 "Failed"
        //             </Status>
        //             <Status color=StatusColor::Info>
        //                 <Icon icon=icondata::AiInfoCircleOutlined class="mr-2" />
        //                 "Pending"
        //             </Status>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Status Sizes"</h2>
        //         <div class="flex flex-wrap items-center gap-4">
        //             <Status size=StatusSize::Xs color=StatusColor::Primary>
        //                 "Extra Small"
        //             </Status>
        //             <Status size=StatusSize::Sm color=StatusColor::Secondary>
        //                 "Small"
        //             </Status>
        //             <Status size=StatusSize::Md color=StatusColor::Accent>
        //                 "Medium"
        //             </Status>
        //             <Status size=StatusSize::Lg color=StatusColor::Success>
        //                 "Large"
        //             </Status>
        //         </div>

        //         <h2 class="text-xl font-semibold">"Server Status Dashboard"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <div class="flex items-center justify-between">
        //                         <h3 class="card-title text-lg">"Web Server"</h3>
        //                         <Status color=StatusColor::Success>
        //                             <Icon icon=icondata::AiCheckCircleOutlined class="mr-1" />
        //                             "Healthy"
        //                         </Status>
        //                     </div>
        //                     <div class="mt-2 text-sm opacity-70">
        //                         <p>"CPU: 45% | Memory: 2.1GB | Uptime: 99.9%"</p>
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <div class="flex items-center justify-between">
        //                         <h3 class="card-title text-lg">"Database"</h3>
        //                         <Status color=StatusColor::Warning>
        //                             <Icon icon=icondata::AiWarningOutlined class="mr-1" />
        //                             "Degraded"
        //                         </Status>
        //                     </div>
        //                     <div class="mt-2 text-sm opacity-70">
        //                         <p>"High latency detected | Connections: 150/200"</p>
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <div class="flex items-center justify-between">
        //                         <h3 class="card-title text-lg">"API Gateway"</h3>
        //                         <Status color=StatusColor::Error>
        //                             <Icon icon=icondata::AiCloseCircleOutlined class="mr-1" />
        //                             "Down"
        //                         </Status>
        //                     </div>
        //                     <div class="mt-2 text-sm opacity-70">
        //                         <p>"Connection timeout | Last seen: 5 minutes ago"</p>
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>

        //         <h2 class="text-xl font-semibold">"User Status"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Team Members"</h3>
        //                 <div class="space-y-3">
        //                     <div class="flex items-center gap-4">
        //                         <Avatar
        //                             src="https://picsum.photos/40/40?random=1"
        //                             alt="John Doe"
        //                             class="w-10 h-10"
        //                         />
        //                         <div class="flex-1">
        //                             <h4 class="font-semibold">"John Doe"</h4>
        //                             <p class="text-sm opacity-70">"Software Engineer"</p>
        //                         </div>
        //                         <Status color=StatusColor::Success size=StatusSize::Sm>
        //                             "Online"
        //                         </Status>
        //                     </div>
        //                     <div class="flex items-center gap-4">
        //                         <Avatar
        //                             src="https://picsum.photos/40/40?random=2"
        //                             alt="Jane Smith"
        //                             class="w-10 h-10"
        //                         />
        //                         <div class="flex-1">
        //                             <h4 class="font-semibold">"Jane Smith"</h4>
        //                             <p class="text-sm opacity-70">"Product Manager"</p>
        //                         </div>
        //                         <Status color=StatusColor::Warning size=StatusSize::Sm>
        //                             "Away"
        //                         </Status>
        //                     </div>
        //                     <div class="flex items-center gap-4">
        //                         <Avatar
        //                             src="https://picsum.photos/40/40?random=3"
        //                             alt="Mike Johnson"
        //                             class="w-10 h-10"
        //                         />
        //                         <div class="flex-1">
        //                             <h4 class="font-semibold">"Mike Johnson"</h4>
        //                             <p class="text-sm opacity-70">"Designer"</p>
        //                         </div>
        //                         <Status color=StatusColor::Error size=StatusSize::Sm>
        //                             "Offline"
        //                         </Status>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Order Status"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Recent Orders"</h3>
        //                 <div class="space-y-4">
        //                     <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
        //                         <div>
        //                             <h4 class="font-semibold">"Order #12345"</h4>
        //                             <p class="text-sm opacity-70">"Wireless Headphones - $99.99"</p>
        //                         </div>
        //                         <Status color=StatusColor::Success>
        //                             <Icon icon=icondata::AiCheckCircleOutlined class="mr-1" />
        //                             "Delivered"
        //                         </Status>
        //                     </div>
        //                     <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
        //                         <div>
        //                             <h4 class="font-semibold">"Order #12346"</h4>
        //                             <p class="text-sm opacity-70">"Laptop Stand - $49.99"</p>
        //                         </div>
        //                         <Status color=StatusColor::Info>
        //                             <Icon icon=icondata::AiTruckOutlined class="mr-1" />
        //                             "Shipped"
        //                         </Status>
        //                     </div>
        //                     <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
        //                         <div>
        //                             <h4 class="font-semibold">"Order #12347"</h4>
        //                             <p class="text-sm opacity-70">"Phone Case - $19.99"</p>
        //                         </div>
        //                         <Status color=StatusColor::Warning>
        //                             <Icon icon=icondata::AiClockCircleOutlined class="mr-1" />
        //                             "Processing"
        //                         </Status>
        //                     </div>
        //                     <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
        //                         <div>
        //                             <h4 class="font-semibold">"Order #12348"</h4>
        //                             <p class="text-sm opacity-70">"Bluetooth Speaker - $79.99"</p>
        //                         </div>
        //                         <Status color=StatusColor::Error>
        //                             <Icon icon=icondata::AiCloseCircleOutlined class="mr-1" />
        //                             "Cancelled"
        //                         </Status>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"Project Status"</h2>
        //         <Card class="bg-base-100 shadow-xl">
        //             <CardBody>
        //                 <h3 class="card-title mb-4">"Active Projects"</h3>
        //                 <div class="space-y-6">
        //                     <div class="border-l-4 border-success pl-4">
        //                         <div class="flex items-center justify-between mb-2">
        //                             <h4 class="font-semibold">"Website Redesign"</h4>
        //                             <Status color=StatusColor::Success size=StatusSize::Sm>
        //                                 "Complete"
        //                             </Status>
        //                         </div>
        //                         <p class="text-sm opacity-70 mb-2">"New company website with modern design"</p>
        //                         <Progress value=100 color=ProgressColor::Success />
        //                         <p class="text-xs opacity-60 mt-1">"100% complete"</p>
        //                     </div>

        //                     <div class="border-l-4 border-warning pl-4">
        //                         <div class="flex items-center justify-between mb-2">
        //                             <h4 class="font-semibold">"Mobile App"</h4>
        //                             <Status color=StatusColor::Warning size=StatusSize::Sm>
        //                                 "In Progress"
        //                             </Status>
        //                         </div>
        //                         <p class="text-sm opacity-70 mb-2">"iOS and Android companion app"</p>
        //                         <Progress value=65 color=ProgressColor::Warning />
        //                         <p class="text-xs opacity-60 mt-1">"65% complete - Testing phase"</p>
        //                     </div>

        //                     <div class="border-l-4 border-info pl-4">
        //                         <div class="flex items-center justify-between mb-2">
        //                             <h4 class="font-semibold">"API Documentation"</h4>
        //                             <Status color=StatusColor::Info size=StatusSize::Sm>
        //                                 "Planned"
        //                             </Status>
        //                         </div>
        //                         <p class="text-sm opacity-70 mb-2">"Comprehensive API documentation"</p>
        //                         <Progress value=25 color=ProgressColor::Info />
        //                         <p class="text-xs opacity-60 mt-1">"25% complete - Planning stage"</p>
        //                     </div>

        //                     <div class="border-l-4 border-error pl-4">
        //                         <div class="flex items-center justify-between mb-2">
        //                             <h4 class="font-semibold">"Legacy Migration"</h4>
        //                             <Status color=StatusColor::Error size=StatusSize::Sm>
        //                                 "Blocked"
        //                             </Status>
        //                         </div>
        //                         <p class="text-sm opacity-70 mb-2">"Migrate legacy systems to new platform"</p>
        //                         <Progress value=10 color=ProgressColor::Error />
        //                         <p class="text-xs opacity-60 mt-1">"10% complete - Waiting for approval"</p>
        //                     </div>
        //                 </div>
        //             </CardBody>
        //         </Card>

        //         <h2 class="text-xl font-semibold">"System Health"</h2>
        //         <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title mb-4">"Service Status"</h3>
        //                     <div class="space-y-3">
        //                         <div class="flex items-center justify-between">
        //                             <div class="flex items-center gap-2">
        //                                 <div class="w-3 h-3 bg-success rounded-full"></div>
        //                                 <span>"Authentication Service"</span>
        //                             </div>
        //                             <Status color=StatusColor::Success size=StatusSize::Xs>
        //                                 "Operational"
        //                             </Status>
        //                         </div>
        //                         <div class="flex items-center justify-between">
        //                             <div class="flex items-center gap-2">
        //                                 <div class="w-3 h-3 bg-success rounded-full"></div>
        //                                 <span>"Payment Processing"</span>
        //                             </div>
        //                             <Status color=StatusColor::Success size=StatusSize::Xs>
        //                                 "Operational"
        //                             </Status>
        //                         </div>
        //                         <div class="flex items-center justify-between">
        //                             <div class="flex items-center gap-2">
        //                                 <div class="w-3 h-3 bg-warning rounded-full"></div>
        //                                 <span>"Email Service"</span>
        //                             </div>
        //                             <Status color=StatusColor::Warning size=StatusSize::Xs>
        //                                 "Degraded"
        //                             </Status>
        //                         </div>
        //                         <div class="flex items-center justify-between">
        //                             <div class="flex items-center gap-2">
        //                                 <div class="w-3 h-3 bg-error rounded-full"></div>
        //                                 <span>"File Storage"</span>
        //                             </div>
        //                             <Status color=StatusColor::Error size=StatusSize::Xs>
        //                                 "Outage"
        //                             </Status>
        //                         </div>
        //                     </div>
        //                 </CardBody>
        //             </Card>

        //             <Card class="bg-base-100 shadow-xl">
        //                 <CardBody>
        //                     <h3 class="card-title mb-4">"Recent Incidents"</h3>
        //                     <div class="space-y-3">
        //                         <div class="flex items-start gap-3">
        //                             <Status color=StatusColor::Error size=StatusSize::Xs>
        //                                 "Critical"
        //                             </Status>
        //                             <div class="flex-1">
        //                                 <h4 class="font-semibold text-sm">"Database Connection Issues"</h4>
        //                                 <p class="text-xs opacity-70">"Resolved - 2 hours ago"</p>
        //                             </div>
        //                         </div>
        //                         <div class="flex items-start gap-3">
        //                             <Status color=StatusColor::Warning size=StatusSize::Xs>
        //                                 "Warning"
        //                             </Status>
        //                             <div class="flex-1">
        //                                 <h4 class="font-semibold text-sm">"High Memory Usage"</h4>
        //                                 <p class="text-xs opacity-70">"Investigating - 1 hour ago"</p>
        //                             </div>
        //                         </div>
        //                         <div class="flex items-start gap-3">
        //                             <Status color=StatusColor::Info size=StatusSize::Xs>
        //                                 "Info"
        //                             </Status>
        //                             <div class="flex-1">
        //                                 <h4 class="font-semibold text-sm">"Scheduled Maintenance"</h4>
        //                                 <p class="text-xs opacity-70">"Completed - 6 hours ago"</p>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 </CardBody>
        //             </Card>
        //         </div>
        //     </div>
        // </div>
    }
}
