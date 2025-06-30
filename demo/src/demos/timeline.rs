use leptos::prelude::*;
use leptos_daisyui_rs::components::*;

#[component]
pub fn TimelineDemo() -> impl IntoView {
    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"Timeline"</h1>
            <p class="text-base-content/70">
                "Timeline component shows a list of events in chronological order"
            </p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic Timeline"</h2>
                <ul class="timeline timeline-vertical">
                    <li>
                        <div class="timeline-start timeline-box">"First Macintosh computer"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"1984"</div>
                        <hr />
                    </li>
                    <li>
                        <hr />
                        <div class="timeline-start timeline-box">"1998"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"iMac"</div>
                        <hr />
                    </li>
                    <li>
                        <hr />
                        <div class="timeline-start timeline-box">"iPod"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"2001"</div>
                        <hr />
                    </li>
                    <li>
                        <hr />
                        <div class="timeline-start timeline-box">"2007"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"iPhone"</div>
                    </li>
                </ul>

                <h2 class="text-xl font-semibold">"Timeline with Colors"</h2>
                <ul class="timeline timeline-vertical">
                    <li>
                        <div class="timeline-start timeline-box">"Write HTML"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-primary w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <hr class="bg-primary" />
                    </li>
                    <li>
                        <hr class="bg-primary" />
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-primary w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"Write CSS"</div>
                        <hr class="bg-primary" />
                    </li>
                    <li>
                        <hr class="bg-primary" />
                        <div class="timeline-start timeline-box">"Write JavaScript"</div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-secondary w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <hr class="bg-secondary" />
                    </li>
                    <li>
                        <hr class="bg-secondary" />
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-accent w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box">"Deploy!"</div>
                    </li>
                </ul>

                <h2 class="text-xl font-semibold">"Project Development Timeline"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"E-commerce Platform Development"</h2>
                        <ul class="timeline timeline-vertical">
                            <li>
                                <div class="timeline-start">
                                    <time class="font-mono italic">"January 2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-primary w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="text-lg font-black">"Project Planning"</div>
                                    "Requirements gathering, wireframes, and tech stack selection"
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <time class="font-mono italic">"February 2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-primary w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="text-lg font-black">"Backend Development"</div>
                                    "API design, database schema, and authentication system"
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <time class="font-mono italic">"March 2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-secondary w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="text-lg font-black">"Frontend Development"</div>
                                    "User interface, shopping cart, and product catalog"
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <time class="font-mono italic">"April 2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-warning w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm-3.536-1.757a.75.75 0 001.05.143l1.486-.992a.75.75 0 000-1.394L7.486 7.607a.75.75 0 00-1.05.143A5.5 5.5 0 0010 4.5c.414 0 .828.046 1.23.134a.75.75 0 10.54-1.402A7 7 0 1010 4.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="text-lg font-black">"Testing Phase"</div>
                                    "Unit tests, integration tests, and user acceptance testing"
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <time class="font-mono italic">"May 2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-success w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="text-lg font-black">"Production Launch"</div>
                                    "Deployment, monitoring, and go-live celebration!"
                                </div>
                            </li>
                        </ul>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Career Timeline"</h2>
                <Card class="bg-gradient-to-r from-indigo-500 to-purple-600 text-white">
                    <CardBody>
                        <h2 class="card-title text-white">"Sarah Johnson - Software Engineer"</h2>
                        <ul class="timeline timeline-vertical">
                            <li>
                                <div class="timeline-start">
                                    <time class="font-mono italic text-indigo-100">"2019"</time>
                                </div>
                                <div class="timeline-middle">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full bg-white/20">
                                            <img
                                                src="https://picsum.photos/40/40?random=5"
                                                alt="University"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-end timeline-box bg-white/10 border-white/20">
                                    <div class="text-lg font-black text-white">
                                        "Computer Science Degree"
                                    </div>
                                    <p class="text-indigo-100">
                                        "Graduated with honors from State University"
                                    </p>
                                </div>
                                <hr class="bg-white/20" />
                            </li>
                            <li>
                                <hr class="bg-white/20" />
                                <div class="timeline-start">
                                    <time class="font-mono italic text-indigo-100">"2020"</time>
                                </div>
                                <div class="timeline-middle">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full bg-white/20">
                                            <img
                                                src="https://picsum.photos/40/40?random=6"
                                                alt="Company"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-end timeline-box bg-white/10 border-white/20">
                                    <div class="text-lg font-black text-white">
                                        "Junior Developer"
                                    </div>
                                    <p class="text-indigo-100">
                                        "Started at TechCorp, working on React applications"
                                    </p>
                                </div>
                                <hr class="bg-white/20" />
                            </li>
                            <li>
                                <hr class="bg-white/20" />
                                <div class="timeline-start">
                                    <time class="font-mono italic text-indigo-100">"2022"</time>
                                </div>
                                <div class="timeline-middle">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full bg-white/20">
                                            <img
                                                src="https://picsum.photos/40/40?random=7"
                                                alt="Promotion"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-end timeline-box bg-white/10 border-white/20">
                                    <div class="text-lg font-black text-white">
                                        "Senior Developer"
                                    </div>
                                    <p class="text-indigo-100">
                                        "Promoted to lead the frontend team"
                                    </p>
                                </div>
                                <hr class="bg-white/20" />
                            </li>
                            <li>
                                <hr class="bg-white/20" />
                                <div class="timeline-start">
                                    <time class="font-mono italic text-indigo-100">"2024"</time>
                                </div>
                                <div class="timeline-middle">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full bg-white/20">
                                            <img
                                                src="https://picsum.photos/40/40?random=8"
                                                alt="Current"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-end timeline-box bg-white/10 border-white/20">
                                    <div class="text-lg font-black text-white">"Tech Lead"</div>
                                    <p class="text-indigo-100">
                                        "Leading architecture decisions and mentoring junior developers"
                                    </p>
                                </div>
                            </li>
                        </ul>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Event Timeline with Status"</h2>
                <ul class="timeline timeline-vertical">
                    <li>
                        <div class="timeline-start timeline-box bg-success text-success-content">
                            <div class="flex items-center gap-2">
                                <Badge
                                    color=Signal::derive(|| BadgeColor::Success)
                                    size=Signal::derive(|| BadgeSize::Sm)
                                >
                                    "✓ Complete"
                                </Badge>
                                <span class="font-semibold">"Design Phase"</span>
                            </div>
                            <p class="text-sm mt-1">"UI/UX mockups and prototypes finished"</p>
                        </div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-success w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end">
                            <time class="font-mono italic">"Week 1-2"</time>
                        </div>
                        <hr class="bg-success" />
                    </li>
                    <li>
                        <hr class="bg-success" />
                        <div class="timeline-start">
                            <time class="font-mono italic">"Week 3-6"</time>
                        </div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-warning w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm-3.536-1.757a.75.75 0 001.05.143l1.486-.992a.75.75 0 000-1.394L7.486 7.607a.75.75 0 00-1.05.143A5.5 5.5 0 0010 4.5c.414 0 .828.046 1.23.134a.75.75 0 10.54-1.402A7 7 0 1010 4.5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box bg-warning text-warning-content">
                            <div class="flex items-center gap-2">
                                <Badge
                                    color=Signal::derive(|| BadgeColor::Warning)
                                    size=Signal::derive(|| BadgeSize::Sm)
                                >
                                    "⚡ In Progress"
                                </Badge>
                                <span class="font-semibold">"Development"</span>
                            </div>
                            <p class="text-sm mt-1">"Building core features - 65% complete"</p>
                            <Progress
                                value=65.0
                                max=100.0
                                color=Signal::derive(|| ProgressColor::Warning)
                                class="w-full mt-2"
                            />
                        </div>
                        <hr class="bg-warning" />
                    </li>
                    <li>
                        <hr class="bg-warning" />
                        <div class="timeline-start timeline-box bg-base-300 text-base-content opacity-60">
                            <div class="flex items-center gap-2">
                                <Badge
                                    color=Signal::derive(|| BadgeColor::Neutral)
                                    size=Signal::derive(|| BadgeSize::Sm)
                                >
                                    "⏳ Pending"
                                </Badge>
                                <span class="font-semibold">"Testing"</span>
                            </div>
                            <p class="text-sm mt-1">"Quality assurance and bug fixes"</p>
                        </div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-base-300 w-5 h-5"
                            >
                                <path
                                    fill-rule="evenodd"
                                    d="M10 18a8 8 0 100-16 8 8 0 000 16zm.75-13a.75.75 0 00-1.5 0v5c0 .414.336.75.75.75h4a.75.75 0 000-1.5h-3.25V5z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </div>
                        <div class="timeline-end">
                            <time class="font-mono italic opacity-60">"Week 7-8"</time>
                        </div>
                        <hr class="bg-base-300" />
                    </li>
                    <li>
                        <hr class="bg-base-300" />
                        <div class="timeline-start">
                            <time class="font-mono italic opacity-60">"Week 9"</time>
                        </div>
                        <div class="timeline-middle">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                                fill="currentColor"
                                class="text-base-300 w-5 h-5"
                            >
                                <path d="M10.75 2.75a.75.75 0 00-1.5 0v8.614L6.295 8.235a.75.75 0 10-1.09 1.03l4.25 4.5a.75.75 0 001.09 0l4.25-4.5a.75.75 0 00-1.09-1.03L10.75 11.364V2.75z" />
                            </svg>
                        </div>
                        <div class="timeline-end timeline-box bg-base-300 text-base-content opacity-60">
                            <div class="flex items-center gap-2">
                                <Badge
                                    color=Signal::derive(|| BadgeColor::Neutral)
                                    size=Signal::derive(|| BadgeSize::Sm)
                                >
                                    "🚀 Planned"
                                </Badge>
                                <span class="font-semibold">"Deployment"</span>
                            </div>
                            <p class="text-sm mt-1">"Production release and monitoring"</p>
                        </div>
                    </li>
                </ul>

                <h2 class="text-xl font-semibold">"Activity Feed Timeline"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Recent Activity"</h2>
                        <ul class="timeline timeline-vertical">
                            <li>
                                <div class="timeline-start">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full">
                                            <img
                                                src="https://picsum.photos/40/40?random=10"
                                                alt="User"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-primary w-5 h-5"
                                    >
                                        <path
                                            fill-rule="evenodd"
                                            d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.236 4.53L8.53 10.347a.75.75 0 00-1.06 1.061l1.5 1.5a.75.75 0 001.137-.089l4-5.5z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="font-semibold">
                                        "John Doe commented on your post"
                                    </div>
                                    <time class="text-xs opacity-50">"2 minutes ago"</time>
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full">
                                            <img
                                                src="https://picsum.photos/40/40?random=11"
                                                alt="User"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-secondary w-5 h-5"
                                    >
                                        <path d="M1 8.25a1.25 1.25 0 112.5 0v7.5a1.25 1.25 0 01-2.5 0v-7.5zM11 3V1.7c0-.268.14-.526.395-.607A2 2 0 0114 3c0 .995-.182 1.948-.514 2.826-.204.54-.836.74-1.206.302A7.999 7.999 0 0010.99 3.75c-.14-.65-.12-1.46.01-2.15z" />
                                        <path d="M9 11.5c0 .368-.14.714-.395.934A2 2 0 016 14c0-.995.182-1.948.514-2.826.204-.54.836-.74 1.206-.302A7.999 7.999 0 009.01 12.25c.14.65.12 1.46-.01 2.15z" />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="font-semibold">"Sarah liked your photo"</div>
                                    <time class="text-xs opacity-50">"5 minutes ago"</time>
                                </div>
                                <hr />
                            </li>
                            <li>
                                <hr />
                                <div class="timeline-start">
                                    <div class="avatar">
                                        <div class="w-10 rounded-full">
                                            <img
                                                src="https://picsum.photos/40/40?random=12"
                                                alt="User"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="timeline-middle">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 20 20"
                                        fill="currentColor"
                                        class="text-accent w-5 h-5"
                                    >
                                        <path d="M3.105 2.289a.75.75 0 00-.826.95l1.414 4.925A1.5 1.5 0 005.135 9.25h6.115a.75.75 0 010 1.5H5.135a1.5 1.5 0 00-1.442 1.086l-1.414 4.926a.75.75 0 00.826.95 28.896 28.896 0 0015.293-7.154.75.75 0 000-1.115A28.897 28.897 0 003.105 2.289z" />
                                    </svg>
                                </div>
                                <div class="timeline-end timeline-box">
                                    <div class="font-semibold">"Alex shared your post"</div>
                                    <time class="text-xs opacity-50">"15 minutes ago"</time>
                                </div>
                            </li>
                        </ul>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}