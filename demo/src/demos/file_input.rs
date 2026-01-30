use leptos::prelude::*;
use leptos_daisyui_rs::components::*;
use wasm_bindgen::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};

#[component]
pub fn FileInputDemo() -> impl IntoView {
    // State for profile picture preview
    let (preview_url, set_preview_url) = signal(Option::<String>::None);

    // File change handler for profile picture
    let handle_file_change = move |ev: Event| {
        let input = ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let reader = FileReader::new().unwrap();
                    let reader_clone = reader.clone();

                    let onload = Closure::wrap(Box::new(move || {
                        if let Ok(result) = reader_clone.result() {
                            if let Some(data_url) = result.as_string() {
                                set_preview_url.set(Some(data_url));
                            }
                        }
                    }) as Box<dyn Fn()>);

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    let _ = reader.read_as_data_url(&file);
                    onload.forget();
                }
            }
        }
    };

    view! {
        <div class="space-y-6">
            <h1 class="text-3xl font-bold">"File Input"</h1>
            <p class="text-base-content/70">"File input is used for uploading files"</p>

            <div class="space-y-4">
                <h2 class="text-xl font-semibold">"Basic File Input"</h2>
                <input type="file" class="file-input w-full max-w-xs" />

                <h2 class="text-xl font-semibold">"File Input with Border"</h2>
                <input type="file" class="file-input file-input-bordered w-full max-w-xs" />

                <h2 class="text-xl font-semibold">"File Input Colors"</h2>
                <div class="space-y-2">
                    <input type="file" class="file-input file-input-bordered w-full max-w-xs" />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-primary w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-secondary w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-accent w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-info w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-success w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-warning w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-error w-full max-w-xs"
                    />
                </div>

                <h2 class="text-xl font-semibold">"File Input Sizes"</h2>
                <div class="space-y-2">
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-xs w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-sm w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-md w-full max-w-xs"
                    />
                    <input
                        type="file"
                        class="file-input file-input-bordered file-input-lg w-full max-w-xs"
                    />
                </div>

                <h2 class="text-xl font-semibold">"Disabled State"</h2>
                <input
                    type="file"
                    class="file-input file-input-bordered w-full max-w-xs"
                    disabled
                />

                <h2 class="text-xl font-semibold">"Multiple File Selection"</h2>
                <input
                    type="file"
                    multiple
                    class="file-input file-input-bordered w-full max-w-xs"
                />

                <h2 class="text-xl font-semibold">"File Type Restrictions"</h2>
                <div class="space-y-2">
                    <div>
                        <label class="label">
                            <span class="label-text">"Images only (.jpg, .png, .gif)"</span>
                        </label>
                        <input
                            type="file"
                            accept="image/*"
                            class="file-input file-input-bordered w-full max-w-xs"
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="label-text">"Documents only (.pdf, .doc, .docx)"</span>
                        </label>
                        <input
                            type="file"
                            accept=".pdf,.doc,.docx"
                            class="file-input file-input-bordered w-full max-w-xs"
                        />
                    </div>
                    <div>
                        <label class="label">
                            <span class="label-text">"Videos only (.mp4, .avi, .mov)"</span>
                        </label>
                        <input
                            type="file"
                            accept="video/*"
                            class="file-input file-input-bordered w-full max-w-xs"
                        />
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"File Upload Form Examples"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <h2 class="card-title">"Profile Picture Upload"</h2>
                            <div class="space-y-4">
                                <div class="flex flex-col sm:flex-row gap-4 items-center">
                                    <div class="avatar">
                                        <div class="w-24 rounded-full ring ring-primary ring-offset-base-100 ring-offset-2">
                                            <img
                                                src="https://picsum.photos/96/96?random=1"
                                                alt="Current avatar"
                                            />
                                        </div>
                                    </div>

                                    {move || {
                                        preview_url
                                            .get()
                                            .map(|url| {
                                                view! {
                                                    <div class="flex flex-col items-center gap-2">
                                                        <div class="text-sm font-semibold text-primary">
                                                            "Preview:"
                                                        </div>
                                                        <div class="avatar">
                                                            <div class="w-24 rounded-full ring ring-success ring-offset-base-100 ring-offset-2">
                                                                <img src=url alt="Preview" />
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            })
                                    }}
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Choose new avatar"</span>
                                    </label>
                                    <input
                                        type="file"
                                        accept="image/*"
                                        class="file-input file-input-bordered file-input-primary w-full"
                                        on:change=handle_file_change
                                    />
                                    <label class="label">
                                        <span class="label-text-alt">
                                            "Max size: 5MB. JPG, PNG only."
                                        </span>
                                    </label>
                                </div>
                                <div class="card-actions justify-end">
                                    <Button size=ButtonSize::Sm style=ButtonStyle::Ghost>
                                        "Cancel"
                                    </Button>
                                    <Button size=ButtonSize::Sm color=ButtonColor::Primary>
                                        "Upload"
                                    </Button>
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card class="bg-base-100 shadow-xl">
                        <CardBody>
                            <h2 class="card-title">"Document Upload"</h2>
                            <div class="space-y-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Upload documents"</span>
                                    </label>
                                    <input
                                        type="file"
                                        multiple
                                        accept=".pdf,.doc,.docx,.txt"
                                        class="file-input file-input-bordered w-full"
                                    />
                                    <label class="label">
                                        <span class="label-text-alt">
                                            "Multiple files allowed. PDF, DOC, DOCX, TXT only."
                                        </span>
                                    </label>
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Description"</span>
                                    </label>
                                    <Textarea
                                        prop:placeholder="Describe these documents..."
                                        class="w-full"
                                    />
                                </div>
                                <div class="card-actions justify-end">
                                    <Button size=ButtonSize::Sm color=ButtonColor::Success>
                                        "Upload Files"
                                    </Button>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>

                <h2 class="text-xl font-semibold">"Advanced Upload Interface"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Project Files Upload"</h2>

                        <div class="space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Project Name"</span>
                                    </label>
                                    <Input attr:placeholder="Enter project name" class="w-full" />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Category"</span>
                                    </label>
                                    <Select class="w-full">
                                        <option disabled selected>
                                            "Select category"
                                        </option>
                                        <option>"Web Development"</option>
                                        <option>"Mobile App"</option>
                                        <option>"Design"</option>
                                        <option>"Marketing"</option>
                                    </Select>
                                </div>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Main Project File"</span>
                                </label>
                                <input
                                    type="file"
                                    accept=".zip,.rar,.tar.gz"
                                    class="file-input file-input-bordered file-input-primary w-full"
                                />
                                <label class="label">
                                    <span class="label-text-alt">
                                        "Archive files only (ZIP, RAR, TAR.GZ). Max 100MB."
                                    </span>
                                </label>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Additional Files"</span>
                                </label>
                                <input
                                    type="file"
                                    multiple
                                    class="file-input file-input-bordered w-full"
                                />
                                <label class="label">
                                    <span class="label-text-alt">
                                        "Optional additional files. Multiple selection allowed."
                                    </span>
                                </label>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Screenshots/Preview Images"</span>
                                </label>
                                <input
                                    type="file"
                                    multiple
                                    accept="image/*"
                                    class="file-input file-input-bordered file-input-secondary w-full"
                                />
                                <label class="label">
                                    <span class="label-text-alt">
                                        "Upload screenshots or preview images. PNG, JPG, GIF allowed."
                                    </span>
                                </label>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Project Description"</span>
                                </label>
                                <textarea
                                    attr:placeholder="Describe your project, its features, and any special instructions..."
                                    rows="4"
                                    class="textarea textarea-bordered w-full"
                                ></textarea>
                            </div>

                            <div class="form-control">
                                <label class="label cursor-pointer justify-start gap-2">
                                    <Checkbox />
                                    <span class="label-text">
                                        "I agree to the terms and conditions"
                                    </span>
                                </label>
                            </div>

                            <div class="card-actions justify-between">
                                <Button style=ButtonStyle::Ghost>"Save as Draft"</Button>
                                <div class="flex gap-2">
                                    <Button style=ButtonStyle::Outline>"Preview"</Button>
                                    <Button color=ButtonColor::Primary>"Submit Project"</Button>
                                </div>
                            </div>
                        </div>
                    </CardBody>
                </Card>

                <h2 class="text-xl font-semibold">"Drag and Drop Style"</h2>
                <div class="border-2 border-dashed border-base-300 rounded-lg p-8 text-center hover:border-primary transition-colors">
                    <div class="space-y-4">
                        <div class="text-4xl opacity-50">"📁"</div>
                        <div>
                            <h3 class="font-semibold">"Drop files here"</h3>
                            <p class="text-sm text-base-content/70">"or click to browse"</p>
                        </div>
                        <input
                            type="file"
                            multiple
                            class="file-input file-input-bordered file-input-primary w-full max-w-xs"
                        />
                        <p class="text-xs text-base-content/50">
                            "Supports: JPG, PNG, PDF, DOC. Max 10MB per file."
                        </p>
                    </div>
                </div>

                <h2 class="text-xl font-semibold">"File Upload with Progress"</h2>
                <Card class="bg-base-100 shadow-xl">
                    <CardBody>
                        <h2 class="card-title">"Upload Progress"</h2>
                        <div class="space-y-4">
                            <input type="file" class="file-input file-input-bordered w-full" />

                            <div class="space-y-2">
                                <div class="flex justify-between text-sm">
                                    <span>"document.pdf"</span>
                                    <span>"75%"</span>
                                </div>
                                <Progress
                                    attr:value=75.0
                                    attr:max=100.0
                                    color=ProgressColor::Primary
                                    class="w-full"
                                />
                            </div>

                            <div class="space-y-2">
                                <div class="flex justify-between text-sm">
                                    <span>"image.jpg"</span>
                                    <span>"100%"</span>
                                </div>
                                <Progress
                                    attr:value=100.0
                                    attr:max=100.0
                                    color=ProgressColor::Success
                                    class="w-full"
                                />
                            </div>

                            <div class="flex justify-between items-center">
                                <span class="text-sm text-base-content/70">
                                    "2 of 3 files uploaded"
                                </span>
                                <Button
                                    size=ButtonSize::Sm
                                    color=ButtonColor::Error
                                    style=ButtonStyle::Outline
                                >
                                    "Cancel"
                                </Button>
                            </div>
                        </div>
                    </CardBody>
                </Card>
            </div>
        </div>
    }
}
