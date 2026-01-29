use leptos::prelude::*;

#[component]
pub fn ListDemo() -> impl IntoView {
    view! {
        // <div class="space-y-6">
        //     <h1 class="text-3xl font-bold">"List"</h1>
        //     <p class="text-base-content/70">
        //         "List component is used to display a series of items"
        //     </p>

        //     <div class="space-y-4">
        //         <h2 class="text-xl font-semibold">"Basic List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>"Item 1"</ListItem>
        //             <ListItem>"Item 2"</ListItem>
        //             <ListItem>"Item 3"</ListItem>
        //             <ListItem>"Item 4"</ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"List with Icons"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <Icon icon=icondata::AiHomeOutlined class="w-5 h-5" />
        //                 "Home"
        //             </ListItem>
        //             <ListItem>
        //                 <Icon icon=icondata::AiUserOutlined class="w-5 h-5" />
        //                 "Profile"
        //             </ListItem>
        //             <ListItem>
        //                 <Icon icon=icondata::AiSettingOutlined class="w-5 h-5" />
        //                 "Settings"
        //             </ListItem>
        //             <ListItem>
        //                 <Icon icon=icondata::AiQuestionCircleOutlined class="w-5 h-5" />
        //                 "Help"
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"List with Actions"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiFileTextOutlined class="w-5 h-5" />
        //                     "Document.pdf"
        //                 </div>
        //                 <div class="flex gap-2">
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiDownloadOutlined class="w-4 h-4" />
        //                     </Button>
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiDeleteOutlined class="w-4 h-4" />
        //                     </Button>
        //                 </div>
        //             </ListItem>
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiFileImageOutlined class="w-5 h-5" />
        //                     "Image.jpg"
        //                 </div>
        //                 <div class="flex gap-2">
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiEyeOutlined class="w-4 h-4" />
        //                     </Button>
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiDownloadOutlined class="w-4 h-4" />
        //                     </Button>
        //                 </div>
        //             </ListItem>
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiFileOutlined class="w-5 h-5" />
        //                     "Spreadsheet.xlsx"
        //                 </div>
        //                 <div class="flex gap-2">
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiEditOutlined class="w-4 h-4" />
        //                     </Button>
        //                     <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
        //                         <Icon icon=icondata::AiShareAltOutlined class="w-4 h-4" />
        //                     </Button>
        //                 </div>
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Contact List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <div class="flex items-center gap-3">
        //                     <Avatar
        //                         src="https://picsum.photos/40/40?random=1"
        //                         alt="Contact"
        //                         class="w-10 h-10"
        //                     />
        //                     <div>
        //                         <div class="font-bold">"John Doe"</div>
        //                         <div class="text-sm opacity-50">"john@example.com"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3">
        //                     <Avatar
        //                         src="https://picsum.photos/40/40?random=2"
        //                         alt="Contact"
        //                         class="w-10 h-10"
        //                     />
        //                     <div>
        //                         <div class="font-bold">"Jane Smith"</div>
        //                         <div class="text-sm opacity-50">"jane@example.com"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3">
        //                     <Avatar
        //                         src="https://picsum.photos/40/40?random=3"
        //                         alt="Contact"
        //                         class="w-10 h-10"
        //                     />
        //                     <div>
        //                         <div class="font-bold">"Bob Johnson"</div>
        //                         <div class="text-sm opacity-50">"bob@example.com"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Notification List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <div class="flex items-start gap-3 w-full">
        //                     <Icon icon=icondata::AiBellOutlined class="w-5 h-5 mt-1 text-primary" />
        //                     <div class="flex-1">
        //                         <div class="font-semibold">"New message received"</div>
        //                         <div class="text-sm opacity-70">"You have a new message from John Doe"</div>
        //                         <div class="text-xs opacity-50 mt-1">"2 minutes ago"</div>
        //                     </div>
        //                     <Badge color=BadgeColor::Primary size=BadgeSize::Sm>"New"</Badge>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-start gap-3 w-full">
        //                     <Icon icon=icondata::AiCheckCircleOutlined class="w-5 h-5 mt-1 text-success" />
        //                     <div class="flex-1">
        //                         <div class="font-semibold">"Task completed"</div>
        //                         <div class="text-sm opacity-70">"Project documentation has been completed"</div>
        //                         <div class="text-xs opacity-50 mt-1">"1 hour ago"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-start gap-3 w-full">
        //                     <Icon icon=icondata::AiExclamationCircleOutlined class="w-5 h-5 mt-1 text-warning" />
        //                     <div class="flex-1">
        //                         <div class="font-semibold">"System maintenance"</div>
        //                         <div class="text-sm opacity-70">"Scheduled maintenance will begin at 2:00 AM"</div>
        //                         <div class="text-xs opacity-50 mt-1">"3 hours ago"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Settings List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiBellOutlined class="w-5 h-5" />
        //                     <div>
        //                         <div class="font-semibold">"Push Notifications"</div>
        //                         <div class="text-sm opacity-70">"Receive push notifications"</div>
        //                     </div>
        //                 </div>
        //                 <Toggle checked=true />
        //             </ListItem>
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiMailOutlined class="w-5 h-5" />
        //                     <div>
        //                         <div class="font-semibold">"Email Notifications"</div>
        //                         <div class="text-sm opacity-70">"Receive email updates"</div>
        //                     </div>
        //                 </div>
        //                 <Toggle checked=false />
        //             </ListItem>
        //             <ListItem class="justify-between">
        //                 <div class="flex items-center gap-3">
        //                     <Icon icon=icondata::AiLockOutlined class="w-5 h-5" />
        //                     <div>
        //                         <div class="font-semibold">"Two-Factor Authentication"</div>
        //                         <div class="text-sm opacity-70">"Enhanced security for your account"</div>
        //                     </div>
        //                 </div>
        //                 <Toggle checked=true />
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Task List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <Checkbox checked=true />
        //                     <div class="flex-1">
        //                         <div class="line-through opacity-50">"Complete project documentation"</div>
        //                         <div class="text-sm opacity-50">"Due: Yesterday"</div>
        //                     </div>
        //                     <Badge color=BadgeColor::Success size=BadgeSize::Sm>"Done"</Badge>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <Checkbox checked=false />
        //                     <div class="flex-1">
        //                         <div class="font-semibold">"Review pull requests"</div>
        //                         <div class="text-sm opacity-70">"Due: Today"</div>
        //                     </div>
        //                     <Badge color=BadgeColor::Warning size=BadgeSize::Sm>"In Progress"</Badge>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <Checkbox checked=false />
        //                     <div class="flex-1">
        //                         <div class="font-semibold">"Prepare presentation"</div>
        //                         <div class="text-sm opacity-70">"Due: Tomorrow"</div>
        //                     </div>
        //                     <Badge color=BadgeColor::Info size=BadgeSize::Sm>"Pending"</Badge>
        //                 </div>
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Product List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <div class="flex items-center gap-4 w-full">
        //                     <div class="w-16 h-16 bg-base-300 rounded-lg flex items-center justify-center">
        //                         <Icon icon=icondata::AiShoppingOutlined class="w-8 h-8" />
        //                     </div>
        //                     <div class="flex-1">
        //                         <div class="font-bold">"Wireless Headphones"</div>
        //                         <div class="text-sm opacity-70">"High-quality sound with noise cancellation"</div>
        //                         <div class="flex items-center gap-2 mt-1">
        //                             <Badge color=BadgeColor::Primary size=BadgeSize::Sm>"Electronics"</Badge>
        //                             <span class="text-lg font-bold text-primary">"$99.99"</span>
        //                         </div>
        //                     </div>
        //                     <Button color=ButtonColor::Primary size=ButtonSize::Sm>
        //                         "Add to Cart"
        //                     </Button>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-4 w-full">
        //                     <div class="w-16 h-16 bg-base-300 rounded-lg flex items-center justify-center">
        //                         <Icon icon=icondata::AiLaptopOutlined class="w-8 h-8" />
        //                     </div>
        //                     <div class="flex-1">
        //                         <div class="font-bold">"Gaming Laptop"</div>
        //                         <div class="text-sm opacity-70">"High-performance laptop for gaming and work"</div>
        //                         <div class="flex items-center gap-2 mt-1">
        //                             <Badge color=BadgeColor::Secondary size=BadgeSize::Sm>"Computers"</Badge>
        //                             <span class="text-lg font-bold text-primary">"$1,299.99"</span>
        //                         </div>
        //                     </div>
        //                     <Button color=ButtonColor::Primary size=ButtonSize::Sm>
        //                         "Add to Cart"
        //                     </Button>
        //                 </div>
        //             </ListItem>
        //         </List>

        //         <h2 class="text-xl font-semibold">"Chat List"</h2>
        //         <List class="bg-base-200 rounded-box">
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <div class="relative">
        //                         <Avatar
        //                             src="https://picsum.photos/48/48?random=10"
        //                             alt="User"
        //                             class="w-12 h-12"
        //                         />
        //                         <div class="absolute bottom-0 right-0 w-3 h-3 bg-success rounded-full border-2 border-base-100"></div>
        //                     </div>
        //                     <div class="flex-1 min-w-0">
        //                         <div class="font-semibold truncate">"Alice Cooper"</div>
        //                         <div class="text-sm opacity-70 truncate">"Hey, how are you doing?"</div>
        //                     </div>
        //                     <div class="text-right">
        //                         <div class="text-xs opacity-50">"2:30 PM"</div>
        //                         <Badge color=BadgeColor::Primary size=BadgeSize::Sm class="mt-1">"3"</Badge>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <div class="relative">
        //                         <Avatar
        //                             src="https://picsum.photos/48/48?random=11"
        //                             alt="User"
        //                             class="w-12 h-12"
        //                         />
        //                         <div class="absolute bottom-0 right-0 w-3 h-3 bg-base-300 rounded-full border-2 border-base-100"></div>
        //                     </div>
        //                     <div class="flex-1 min-w-0">
        //                         <div class="font-semibold truncate">"Bob Wilson"</div>
        //                         <div class="text-sm opacity-70 truncate">"Can we schedule a meeting for tomorrow?"</div>
        //                     </div>
        //                     <div class="text-right">
        //                         <div class="text-xs opacity-50">"1:15 PM"</div>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //             <ListItem>
        //                 <div class="flex items-center gap-3 w-full">
        //                     <div class="relative">
        //                         <Avatar
        //                             src="https://picsum.photos/48/48?random=12"
        //                             alt="User"
        //                             class="w-12 h-12"
        //                         />
        //                         <div class="absolute bottom-0 right-0 w-3 h-3 bg-warning rounded-full border-2 border-base-100"></div>
        //                     </div>
        //                     <div class="flex-1 min-w-0">
        //                         <div class="font-semibold truncate">"Carol Davis"</div>
        //                         <div class="text-sm opacity-70 truncate">"The project looks great! 👏"</div>
        //                     </div>
        //                     <div class="text-right">
        //                         <div class="text-xs opacity-50">"11:45 AM"</div>
        //                         <Badge color=BadgeColor::Success size=BadgeSize::Sm class="mt-1">"1"</Badge>
        //                     </div>
        //                 </div>
        //             </ListItem>
        //         </List>
        //     </div>
        // </div>
    }
}
