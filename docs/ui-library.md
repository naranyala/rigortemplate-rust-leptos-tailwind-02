# UI Library

The UI library provides a set of reusable components built with Leptos and Tailwind CSS.

## Components

### DataTable
A generic, sortable table for displaying collections of data.
- **Props**: `data` (ReadSignal), `columns` (Vec of Column definitions).
- **Features**: Client-side sorting, customizable columns.

### Tooltip
A simple hover-triggered tooltip.
- **Props**: `children` (component to wrap), `text` (tooltip content).

### Progress
A visual progress bar.
- **Props**: `value` (ReadSignal), `max` (f64), `class` (optional custom styles).

### Stepper
A multi-step process indicator.
- **Props**: `steps` (Vec of Step), `current_step` (ReadSignal), `on_step_change` (WriteSignal).

### FormInput
A validated text input field.
- **Props**: `label`, `value` (RwSignal), `placeholder`, `rules` (Vec of ValidationRule), `input_type`.
- **Validation**: Uses regex-based rules to show error messages in real-time.

### MultiSelect
A dropdown for selecting multiple options.
- **Props**: `options` (Vec of value/label pairs), `selected` (RwSignal), `placeholder`.
