use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn StatusDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Status"
            description="Status indicators to show different states and conditions"
        >
            <Section title="Basic Status Indicators" row=true>
                <div class="flex items-center gap-2">
                    <Status color=StatusColor::Success />
                    <span>"Online"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status color=StatusColor::Warning />
                    <span>"Away"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status color=StatusColor::Error />
                    <span>"Offline"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status color=StatusColor::Info />
                    <span>"Idle"</span>
                </div>
            </Section>

            <Section title="Status Badges with Text" row=true>
                <Badge color=BadgeColor::Success>
                    <Icon icon=icondata::AiCheckCircleOutlined />
                    " Completed"
                </Badge>
                <Badge color=BadgeColor::Warning>
                    <Icon icon=icondata::AiClockCircleOutlined />
                    " In Progress"
                </Badge>
                <Badge color=BadgeColor::Error>
                    <Icon icon=icondata::AiCloseCircleOutlined />
                    " Failed"
                </Badge>
                <Badge color=BadgeColor::Info>
                    <Icon icon=icondata::AiInfoCircleOutlined />
                    " Pending"
                </Badge>
            </Section>

            <Section title="Status Sizes" row=true>
                <div class="flex items-center gap-2">
                    <Status size=StatusSize::Xs color=StatusColor::Primary />
                    <span class="text-xs">"Extra Small"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status size=StatusSize::Sm color=StatusColor::Secondary />
                    <span class="text-sm">"Small"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status size=StatusSize::Md color=StatusColor::Accent />
                    <span>"Medium"</span>
                </div>
                <div class="flex items-center gap-2">
                    <Status size=StatusSize::Lg color=StatusColor::Success />
                    <span class="text-lg">"Large"</span>
                </div>
            </Section>

            <Section title="Server Status Dashboard">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <div class="flex items-center justify-between">
                                <h3 class="card-title text-lg">"Web Server"</h3>
                                <Badge color=BadgeColor::Success>
                                    <Icon icon=icondata::AiCheckCircleOutlined />
                                    " Healthy"
                                </Badge>
                            </div>
                            <div class="mt-2 text-sm opacity-70">
                                <p>"CPU: 45% | Memory: 2.1GB | Uptime: 99.9%"</p>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <div class="flex items-center justify-between">
                                <h3 class="card-title text-lg">"Database"</h3>
                                <Badge color=BadgeColor::Warning>
                                    <Icon icon=icondata::AiWarningOutlined />
                                    " Degraded"
                                </Badge>
                            </div>
                            <div class="mt-2 text-sm opacity-70">
                                <p>"High latency detected | Connections: 150/200"</p>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <div class="flex items-center justify-between">
                                <h3 class="card-title text-lg">"API Gateway"</h3>
                                <Badge color=BadgeColor::Error>
                                    <Icon icon=icondata::AiCloseCircleOutlined />
                                    " Down"
                                </Badge>
                            </div>
                            <div class="mt-2 text-sm opacity-70">
                                <p>"Connection timeout | Last seen: 5 minutes ago"</p>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </Section>

            <Section title="User Status">
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h3 class="card-title mb-4">"Team Members"</h3>
                        <div class="space-y-3">
                            <div class="flex items-center gap-4">
                                <Avatar>
                                    <div class="w-10 h-10 rounded-full">
                                        <img
                                            src="https://picsum.photos/40/40?random=1"
                                            alt="John Doe"
                                        />
                                    </div>
                                </Avatar>
                                <div class="flex-1">
                                    <h4 class="font-semibold">"John Doe"</h4>
                                    <p class="text-sm opacity-70">"Software Engineer"</p>
                                </div>
                                <div class="flex items-center gap-2">
                                    <Status color=StatusColor::Success size=StatusSize::Sm />
                                    <span class="text-sm">"Online"</span>
                                </div>
                            </div>
                            <div class="flex items-center gap-4">
                                <Avatar>
                                    <div class="w-10 h-10 rounded-full">
                                        <img
                                            src="https://picsum.photos/40/40?random=2"
                                            alt="Jane Smith"
                                        />
                                    </div>
                                </Avatar>
                                <div class="flex-1">
                                    <h4 class="font-semibold">"Jane Smith"</h4>
                                    <p class="text-sm opacity-70">"Product Manager"</p>
                                </div>
                                <div class="flex items-center gap-2">
                                    <Status color=StatusColor::Warning size=StatusSize::Sm />
                                    <span class="text-sm">"Away"</span>
                                </div>
                            </div>
                            <div class="flex items-center gap-4">
                                <Avatar>
                                    <div class="w-10 h-10 rounded-full">
                                        <img
                                            src="https://picsum.photos/40/40?random=3"
                                            alt="Mike Johnson"
                                        />
                                    </div>
                                </Avatar>
                                <div class="flex-1">
                                    <h4 class="font-semibold">"Mike Johnson"</h4>
                                    <p class="text-sm opacity-70">"Designer"</p>
                                </div>
                                <div class="flex items-center gap-2">
                                    <Status color=StatusColor::Error size=StatusSize::Sm />
                                    <span class="text-sm">"Offline"</span>
                                </div>
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Order Status">
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h3 class="card-title mb-4">"Recent Orders"</h3>
                        <div class="space-y-4">
                            <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
                                <div>
                                    <h4 class="font-semibold">"Order #12345"</h4>
                                    <p class="text-sm opacity-70">
                                        "Wireless Headphones - $99.99"
                                    </p>
                                </div>
                                <Badge color=BadgeColor::Success>
                                    <Icon icon=icondata::AiCheckCircleOutlined />
                                    " Delivered"
                                </Badge>
                            </div>
                            <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
                                <div>
                                    <h4 class="font-semibold">"Order #12346"</h4>
                                    <p class="text-sm opacity-70">"Laptop Stand - $49.99"</p>
                                </div>
                                <Badge color=BadgeColor::Info>
                                    <Icon icon=icondata::AiTruckOutlined />
                                    " Shipped"
                                </Badge>
                            </div>
                            <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
                                <div>
                                    <h4 class="font-semibold">"Order #12347"</h4>
                                    <p class="text-sm opacity-70">"Phone Case - $19.99"</p>
                                </div>
                                <Badge color=BadgeColor::Warning>
                                    <Icon icon=icondata::AiClockCircleOutlined />
                                    " Processing"
                                </Badge>
                            </div>
                            <div class="flex items-center justify-between p-4 bg-base-200 rounded-lg">
                                <div>
                                    <h4 class="font-semibold">"Order #12348"</h4>
                                    <p class="text-sm opacity-70">
                                        "Bluetooth Speaker - $79.99"
                                    </p>
                                </div>
                                <Badge color=BadgeColor::Error>
                                    <Icon icon=icondata::AiCloseCircleOutlined />
                                    " Cancelled"
                                </Badge>
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </Section>

            <Section title="Project Status">
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h3 class="card-title mb-4">"Active Projects"</h3>
                        <div class="space-y-6">
                            <div class="border-l-4 border-success pl-4">
                                <div class="flex items-center justify-between mb-2">
                                    <h4 class="font-semibold">"Website Redesign"</h4>
                                    <Badge color=BadgeColor::Success size=BadgeSize::Sm>
                                        "Complete"
                                    </Badge>
                                </div>
                                <p class="text-sm opacity-70 mb-2">
                                    "New company website with modern design"
                                </p>
                                <Progress
                                    attr:value=100.0
                                    attr:max=100.0
                                    color=ProgressColor::Success
                                    class="w-full"
                                />
                                <p class="text-xs opacity-60 mt-1">"100% complete"</p>
                            </div>

                            <div class="border-l-4 border-warning pl-4">
                                <div class="flex items-center justify-between mb-2">
                                    <h4 class="font-semibold">"Mobile App"</h4>
                                    <Badge color=BadgeColor::Warning size=BadgeSize::Sm>
                                        "In Progress"
                                    </Badge>
                                </div>
                                <p class="text-sm opacity-70 mb-2">
                                    "iOS and Android companion app"
                                </p>
                                <Progress
                                    attr:value=65.0
                                    attr:max=100.0
                                    color=ProgressColor::Warning
                                    class="w-full"
                                />
                                <p class="text-xs opacity-60 mt-1">
                                    "65% complete - Testing phase"
                                </p>
                            </div>

                            <div class="border-l-4 border-info pl-4">
                                <div class="flex items-center justify-between mb-2">
                                    <h4 class="font-semibold">"API Documentation"</h4>
                                    <Badge color=BadgeColor::Info size=BadgeSize::Sm>
                                        "Planned"
                                    </Badge>
                                </div>
                                <p class="text-sm opacity-70 mb-2">
                                    "Comprehensive API documentation"
                                </p>
                                <Progress
                                    attr:value=25.0
                                    attr:max=100.0
                                    color=ProgressColor::Info
                                    class="w-full"
                                />
                                <p class="text-xs opacity-60 mt-1">
                                    "25% complete - Planning stage"
                                </p>
                            </div>

                            <div class="border-l-4 border-error pl-4">
                                <div class="flex items-center justify-between mb-2">
                                    <h4 class="font-semibold">"Legacy Migration"</h4>
                                    <Badge color=BadgeColor::Error size=BadgeSize::Sm>
                                        "Blocked"
                                    </Badge>
                                </div>
                                <p class="text-sm opacity-70 mb-2">
                                    "Migrate legacy systems to new platform"
                                </p>
                                <Progress
                                    attr:value=10.0
                                    attr:max=100.0
                                    color=ProgressColor::Error
                                    class="w-full"
                                />
                                <p class="text-xs opacity-60 mt-1">
                                    "10% complete - Waiting for approval"
                                </p>
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </Section>

            <Section title="System Health">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <h3 class="card-title mb-4">"Service Status"</h3>
                            <div class="space-y-3">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <Status color=StatusColor::Success size=StatusSize::Sm />
                                        <span>"Authentication Service"</span>
                                    </div>
                                    <Badge color=BadgeColor::Success size=BadgeSize::Xs>
                                        "Operational"
                                    </Badge>
                                </div>
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <Status color=StatusColor::Success size=StatusSize::Sm />
                                        <span>"Payment Processing"</span>
                                    </div>
                                    <Badge color=BadgeColor::Success size=BadgeSize::Xs>
                                        "Operational"
                                    </Badge>
                                </div>
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <Status color=StatusColor::Warning size=StatusSize::Sm />
                                        <span>"Email Service"</span>
                                    </div>
                                    <Badge color=BadgeColor::Warning size=BadgeSize::Xs>
                                        "Degraded"
                                    </Badge>
                                </div>
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <Status color=StatusColor::Error size=StatusSize::Sm />
                                        <span>"File Storage"</span>
                                    </div>
                                    <Badge color=BadgeColor::Error size=BadgeSize::Xs>
                                        "Outage"
                                    </Badge>
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <h3 class="card-title mb-4">"Recent Incidents"</h3>
                            <div class="space-y-3">
                                <div class="flex items-start gap-3">
                                    <Badge color=BadgeColor::Error size=BadgeSize::Xs>
                                        "Critical"
                                    </Badge>
                                    <div class="flex-1">
                                        <h4 class="font-semibold text-sm">
                                            "Database Connection Issues"
                                        </h4>
                                        <p class="text-xs opacity-70">"Resolved - 2 hours ago"</p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <Badge color=BadgeColor::Warning size=BadgeSize::Xs>
                                        "Warning"
                                    </Badge>
                                    <div class="flex-1">
                                        <h4 class="font-semibold text-sm">"High Memory Usage"</h4>
                                        <p class="text-xs opacity-70">
                                            "Investigating - 1 hour ago"
                                        </p>
                                    </div>
                                </div>
                                <div class="flex items-start gap-3">
                                    <Badge color=BadgeColor::Info size=BadgeSize::Xs>
                                        "Info"
                                    </Badge>
                                    <div class="flex-1">
                                        <h4 class="font-semibold text-sm">
                                            "Scheduled Maintenance"
                                        </h4>
                                        <p class="text-xs opacity-70">"Completed - 6 hours ago"</p>
                                    </div>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </Section>
        </ContentLayout>
    }
}
