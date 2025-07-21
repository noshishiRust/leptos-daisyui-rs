use crate::core::{ContentLayout, Section};
use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use leptos_icons::Icon;

#[component]
pub fn StatsDemo() -> impl IntoView {
    view! {
        <ContentLayout
            title="Stats"
            description="Stats component is used to show statistics with numbers and descriptions"
        >
            <Section title="Basic Stat">
                <Stats class="shadow">
                    <Stat>
                        <StatTitle>"Total Page Views"</StatTitle>
                        <StatValue>"89,400"</StatValue>
                        <StatDesc>"21% more than last month"</StatDesc>
                    </Stat>
                </Stats>
            </Section>

            <Section title="Multiple Stats">
                <Stats class="shadow">
                    <Stat>
                        <StatFigure class="text-primary">
                            <Icon icon=icondata::AiHeartOutlined />
                        </StatFigure>
                        <StatTitle>"Total Likes"</StatTitle>
                        <StatValue class="text-primary">"25.6K"</StatValue>
                        <StatDesc>"21% more than last month"</StatDesc>
                    </Stat>
                    <Stat>
                        <StatFigure class="text-secondary">
                            <Icon icon=icondata::AiThunderboltOutlined />
                        </StatFigure>
                        <StatTitle>"Page Views"</StatTitle>
                        <StatValue class="text-secondary">"2.6M"</StatValue>
                        <StatDesc>"21% more than last month"</StatDesc>
                    </Stat>
                    <Stat>
                        <StatFigure class="text-secondary">
                            <Avatar modifier=AvatarModifier::Online>
                                <img src="https://picsum.photos/64/64?random=1" alt="User" />
                            </Avatar>
                        </StatFigure>
                        <StatValue>"86%"</StatValue>
                        <StatTitle>"Tasks done"</StatTitle>
                        <StatDesc class="text-secondary">"31 tasks remaining"</StatDesc>
                    </Stat>
                </Stats>
            </Section>

            <Section title="Vertical Stats">
                <Stats vertical=RwSignal::new(true) class="shadow lg:!flex-row">
                    <Stat>
                        <StatTitle>"Downloads"</StatTitle>
                        <StatValue>"31K"</StatValue>
                        <StatDesc>"Jan 1st - Feb 1st"</StatDesc>
                    </Stat>
                    <Stat>
                        <StatTitle>"New Users"</StatTitle>
                        <StatValue>"4,200"</StatValue>
                        <StatDesc>"↗︎ 400 (22%)"</StatDesc>
                    </Stat>
                    <Stat>
                        <StatTitle>"New Registers"</StatTitle>
                        <StatValue>"1,200"</StatValue>
                        <StatDesc>"↘︎ 90 (14%)"</StatDesc>
                    </Stat>
                </Stats>
            </Section>

            <Section title="Stats with Actions">
                <Stats class="shadow">
                    <Stat>
                        <StatFigure class="text-primary">
                            <Icon icon=icondata::AiCreditCardOutlined />
                        </StatFigure>
                        <StatTitle>"Account balance"</StatTitle>
                        <StatValue>"$89,400"</StatValue>
                        <StatActions>
                            <Button size=ButtonSize::Sm color=ButtonColor::Success>
                                "Add funds"
                            </Button>
                        </StatActions>
                    </Stat>
                    <Stat>
                        <StatFigure class="text-secondary">
                            <Icon icon=icondata::AiSettingOutlined />
                        </StatFigure>
                        <StatTitle>"Current balance"</StatTitle>
                        <StatValue>"$89,400"</StatValue>
                        <StatActions>
                            <Button size=ButtonSize::Sm>"Withdrawal"</Button>
                            <Button size=ButtonSize::Sm color=ButtonColor::Primary>
                                "Deposit"
                            </Button>
                        </StatActions>
                    </Stat>
                </Stats>
            </Section>

            <Section title="Dashboard Grid">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                    <Stats class="shadow">
                        <Stat class="bg-primary text-primary-content">
                            <StatFigure class="text-primary-content">
                                <Icon icon=icondata::AiInfoCircleOutlined />
                            </StatFigure>
                            <StatTitle class="text-primary-content">"Total Orders"</StatTitle>
                            <StatValue>"31K"</StatValue>
                            <StatDesc class="text-primary-content">"Jan 1st - Feb 1st"</StatDesc>
                        </Stat>
                    </Stats>
                    <Stats class="shadow">
                        <Stat class="bg-secondary text-secondary-content">
                            <StatFigure class="text-secondary-content">
                                <Icon icon=icondata::AiSettingOutlined />
                            </StatFigure>
                            <StatTitle class="text-secondary-content">"Revenue"</StatTitle>
                            <StatValue>"$4,200"</StatValue>
                            <StatDesc class="text-secondary-content">"↗︎ 400 (22%)"</StatDesc>
                        </Stat>
                    </Stats>
                    <Stats class="shadow">
                        <Stat class="bg-accent text-accent-content">
                            <StatFigure class="text-accent-content">
                                <Icon icon=icondata::AiCreditCardOutlined />
                            </StatFigure>
                            <StatTitle class="text-accent-content">"New Users"</StatTitle>
                            <StatValue>"1,200"</StatValue>
                            <StatDesc class="text-accent-content">"↘︎ 90 (14%)"</StatDesc>
                        </Stat>
                    </Stats>
                    <Stats class="shadow">
                        <Stat class="bg-info text-info-content">
                            <StatFigure class="text-info-content">
                                <Icon icon=icondata::AiBarChartOutlined />
                            </StatFigure>
                            <StatTitle class="text-info-content">"Conversion Rate"</StatTitle>
                            <StatValue>"86%"</StatValue>
                            <StatDesc class="text-info-content">"↗︎ 8% this month"</StatDesc>
                        </Stat>
                    </Stats>
                </div>
            </Section>

            <Section title="E-commerce Dashboard">
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Sales Dashboard"</h2>
                        <Stats vertical=RwSignal::new(true) class="shadow lg:!flex-row">
                            <Stat>
                                <StatFigure class="text-success">
                                    <Icon icon=icondata::AiLineChartOutlined />
                                </StatFigure>
                                <StatTitle>"Sales"</StatTitle>
                                <StatValue class="text-success">"$125,430"</StatValue>
                                <StatDesc>"↗︎ 12% increase"</StatDesc>
                            </Stat>
                            <Stat>
                                <StatFigure class="text-warning">
                                    <Icon icon=icondata::AiShoppingCartOutlined />
                                </StatFigure>
                                <StatTitle>"Orders"</StatTitle>
                                <StatValue class="text-warning">"2,847"</StatValue>
                                <StatDesc>"↗︎ 5% increase"</StatDesc>
                            </Stat>
                            <Stat>
                                <StatFigure class="text-error">
                                    <Icon icon=icondata::AiShopOutlined />
                                </StatFigure>
                                <StatTitle>"Products"</StatTitle>
                                <StatValue class="text-error">"1,264"</StatValue>
                                <StatDesc>"↘︎ 3% decrease"</StatDesc>
                            </Stat>
                            <Stat>
                                <StatFigure class="text-info">
                                    <Icon icon=icondata::AiTeamOutlined />
                                </StatFigure>
                                <StatTitle>"Customers"</StatTitle>
                                <StatValue class="text-info">"15,672"</StatValue>
                                <StatDesc>"↗︎ 18% increase"</StatDesc>
                            </Stat>
                        </Stats>
                    </CardBody>
                </Card>
            </Section>
        </ContentLayout>
    }
}
