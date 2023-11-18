#include "util/gui/widget.h"

#include <google/protobuf/message.h>
#include <stdint.h>

#include "absl/log/log.h"
#include "imgui.h"
#include "misc/cpp/imgui_stdlib.h"
#include "proto/gui_extension.pb.h"
#include "util/gui/flags.h"
namespace gui {
namespace {
using CppType = google::protobuf::FieldDescriptor::CppType;
constexpr CppType NoType = static_cast<CppType>(0);
const std::string& empty() {
    static const std::string e;
    return e;
}
}  // namespace

ProtoGui ProtoGui::Begin(google::protobuf::Message* message, bool* p_open) {
    const auto* desc = message->GetDescriptor();
    const auto& begin = desc->options().GetExtension(proto::begin);
    bool val = false;
    int pushes = 0;
    if (desc->options().HasExtension(proto::style)) {
        pushes = PushStyleVar(desc->options().GetExtension(proto::style));
    }
    switch (begin.begin_case()) {
        case proto::Begin::BeginCase::kWindow: {
            const auto& window = begin.window();
            const auto& name =
                !window.name().empty() ? window.name() : desc->name();
            const auto& flags = GetWindowFlags(window.flags());
            val = ImGui::Begin(name, p_open, flags);
            break;
        }
        case proto::Begin::BeginCase::kChild: {
            const auto& child = begin.child();
            const auto& flags = GetWindowFlags(child.flags());
            val = ImGui::BeginChild(child.id(), ImVec2(child.w(), child.h()),
                                    child.border(), flags);
            break;
        }
        case proto::Begin::BeginCase::kGrp: {
            ImGui::BeginGroup();
            val = true;
            break;
        }
        case proto::Begin::BeginCase::kPopup: {
            const auto& popup = begin.popup();
            const auto& flags = GetPopupFlags(popup.flags());
            switch (popup.context()) {
                case proto::PopupContext::NONE:
                    val = ImGui::BeginPopup(popup.id(), flags);
                    break;
                case proto::PopupContext::ITEM:
                    val = ImGui::BeginPopupContextItem(popup.id(), flags);
                    break;
                case proto::PopupContext::WINDOW:
                    val = ImGui::BeginPopupContextWindow(popup.id(), flags);
                    break;
                case proto::PopupContext::VOID:
                    val = ImGui::BeginPopupContextVoid(popup.id(), flags);
                    break;
                default:
                    /* do nothing */;
            }
            break;
        }
        default:
            /* do nothing */;
    }

    int colors = 0;
    if (desc->options().HasExtension(proto::color)) {
        colors = PushStyleColor(desc->options().GetExtension(proto::color));
    }

    int columns = 0;
    if (val && desc->options().HasExtension(proto::table)) {
        const auto& table = desc->options().GetExtension(proto::table);
        ImVec2 outer_size(0.0f, 0.0f);
        if (table.outer_size().size() == 2) {
            outer_size = ImVec2(table.outer_size(0), table.outer_size(1));
        }
        columns = table.column();
        if (columns <= 0) columns = -1;
        val = ImGui::BeginTable(table.id(), columns == -1 ? 2 : columns,
                                GetTableFlags(table.flags()), outer_size,
                                table.inner_width());
        for (const auto& header : table.header()) {
            ImGui::TableSetupColumn(header);
        }
        if (table.header_size()) ImGui::TableHeadersRow();
    }

    float item_width = 0.0;
    if (desc->options().HasExtension(proto::default_width)) {
        ImGui::PushItemWidth(
            desc->options().GetExtension(proto::default_width));
    }

    ProtoGui pg(val, message, pushes, colors, item_width);
    pg.columns_ = columns;
    return pg;
}

ImGuiDataType GetDataType(const google::protobuf::FieldDescriptor* field) {
    switch (field->cpp_type()) {
        case CppType::CPPTYPE_INT32:
            return ImGuiDataType_S32;
        case CppType::CPPTYPE_INT64:
            return ImGuiDataType_S64;
        case CppType::CPPTYPE_UINT32:
            return ImGuiDataType_U32;
        case CppType::CPPTYPE_UINT64:
            return ImGuiDataType_U64;
        case CppType::CPPTYPE_FLOAT:
            return ImGuiDataType_Float;
        case CppType::CPPTYPE_DOUBLE:
            return ImGuiDataType_Double;
        default:
            return -1;
    }
}

union Scalar {
    int64_t s64;
    int32_t s32;
    uint64_t u64;
    uint32_t u32;
    float f;
    double d;

    static Scalar From(const proto::Scalar& val, CppType type = NoType) {
        Scalar ret;
        CppType found = NoType;
        switch (val.scalar_case()) {
            case proto::Scalar::ScalarCase::kS32:
                found = CppType::CPPTYPE_INT32;
                ret.s32 = val.s32();
                break;
            case proto::Scalar::ScalarCase::kS64:
                found = CppType::CPPTYPE_INT64;
                ret.s64 = val.s64();
                break;
            case proto::Scalar::ScalarCase::kU32:
                found = CppType::CPPTYPE_UINT32;
                ret.u32 = val.u32();
                break;
            case proto::Scalar::ScalarCase::kU64:
                found = CppType::CPPTYPE_UINT64;
                ret.u64 = val.u64();
                break;
            case proto::Scalar::ScalarCase::kF32:
                found = CppType::CPPTYPE_FLOAT;
                ret.f = val.f32();
                break;
            case proto::Scalar::ScalarCase::kF64:
                found = CppType::CPPTYPE_DOUBLE;
                ret.d = val.f64();
                break;
            default:
                LOG(ERROR) << "Scalar::From: no value set";
                ret.s64 = -1;
        }
        if (type && type != found) {
            LOG(ERROR) << "Scalar::From: found type " << found
                       << ", but wanted type " << type;
        }
        return ret;
    }

    static Scalar Get(const google::protobuf::Reflection* reflection,
                      google::protobuf::Message* message,
                      const google::protobuf::FieldDescriptor* field) {
        Scalar ret;
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_INT32:
                ret.s32 = reflection->GetInt32(*message, field);
                break;
            case CppType::CPPTYPE_INT64:
                ret.s64 = reflection->GetInt64(*message, field);
                break;
            case CppType::CPPTYPE_UINT32:
                ret.u32 = reflection->GetUInt32(*message, field);
                break;
            case CppType::CPPTYPE_UINT64:
                ret.u64 = reflection->GetUInt64(*message, field);
                break;
            case CppType::CPPTYPE_FLOAT:
                ret.f = reflection->GetFloat(*message, field);
                break;
            case CppType::CPPTYPE_DOUBLE:
                ret.d = reflection->GetDouble(*message, field);
                break;
            default:
                LOG(ERROR) << "Scalar::Get called on a non-scalar field: "
                           << field->name();
                ret.s64 = -1;
        }
        return ret;
    }

    static Scalar GetRepeated(const google::protobuf::Reflection* reflection,
                              google::protobuf::Message* message,
                              const google::protobuf::FieldDescriptor* field,
                              int i) {
        Scalar ret;
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_INT32:
                ret.s32 = reflection->GetRepeatedInt32(*message, field, i);
                break;
            case CppType::CPPTYPE_INT64:
                ret.s64 = reflection->GetRepeatedInt64(*message, field, i);
                break;
            case CppType::CPPTYPE_UINT32:
                ret.u32 = reflection->GetRepeatedUInt32(*message, field, i);
                break;
            case CppType::CPPTYPE_UINT64:
                ret.u64 = reflection->GetRepeatedUInt64(*message, field, i);
                break;
            case CppType::CPPTYPE_FLOAT:
                ret.f = reflection->GetRepeatedFloat(*message, field, i);
                break;
            case CppType::CPPTYPE_DOUBLE:
                ret.d = reflection->GetRepeatedDouble(*message, field, i);
                break;
            default:
                LOG(ERROR)
                    << "Scalar::GetRepeated called on a non-scalar field: "
                    << field->name();
                ret.s64 = -1;
        }
        return ret;
    }

    static void GetRepeated(const google::protobuf::Reflection* reflection,
                            google::protobuf::Message* message,
                            const google::protobuf::FieldDescriptor* field,
                            void* data, int n) {
        char* ptr = (char*)data;
        for (int i = 0; i < n; ++i) {
            switch (field->cpp_type()) {
                case CppType::CPPTYPE_INT32: {
                    int32_t* val = (int32_t*)ptr;
                    *val = reflection->GetRepeatedInt32(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                case CppType::CPPTYPE_INT64: {
                    int64_t* val = (int64_t*)ptr;
                    *val = reflection->GetRepeatedInt64(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                case CppType::CPPTYPE_UINT32: {
                    uint32_t* val = (uint32_t*)ptr;
                    *val = reflection->GetRepeatedUInt32(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                case CppType::CPPTYPE_UINT64: {
                    uint64_t* val = (uint64_t*)ptr;
                    *val = reflection->GetRepeatedUInt64(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                case CppType::CPPTYPE_FLOAT: {
                    float* val = (float*)ptr;
                    *val = reflection->GetRepeatedFloat(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                case CppType::CPPTYPE_DOUBLE: {
                    double* val = (double*)ptr;
                    *val = reflection->GetRepeatedDouble(*message, field, i);
                    ptr += sizeof(*val);
                    break;
                }
                default:
                    LOG(ERROR)
                        << "Scalar::GetRepeated called on a non-scalar field: "
                        << field->name();
            }
        }
    }

    void Set(const google::protobuf::Reflection* reflection,
             google::protobuf::Message* message,
             const google::protobuf::FieldDescriptor* field) {
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_INT32:
                reflection->SetInt32(message, field, this->s32);
                break;
            case CppType::CPPTYPE_INT64:
                reflection->SetInt64(message, field, this->s64);
                break;
            case CppType::CPPTYPE_UINT32:
                reflection->SetUInt32(message, field, this->u32);
                break;
            case CppType::CPPTYPE_UINT64:
                reflection->SetUInt64(message, field, this->u64);
                break;
            case CppType::CPPTYPE_FLOAT:
                reflection->SetFloat(message, field, this->f);
                break;
            case CppType::CPPTYPE_DOUBLE:
                reflection->SetDouble(message, field, this->d);
                break;
            default:
                LOG(ERROR) << "Scalar::Set called on a non-scalar field: "
                           << field->name();
        }
    }
    void SetRepeated(const google::protobuf::Reflection* reflection,
                     google::protobuf::Message* message,
                     const google::protobuf::FieldDescriptor* field, int i) {
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_INT32:
                reflection->SetRepeatedInt32(message, field, i, this->s32);
                break;
            case CppType::CPPTYPE_INT64:
                reflection->SetRepeatedInt64(message, field, i, this->s64);
                break;
            case CppType::CPPTYPE_UINT32:
                reflection->SetRepeatedUInt32(message, field, i, this->u32);
                break;
            case CppType::CPPTYPE_UINT64:
                reflection->SetRepeatedUInt64(message, field, i, this->u64);
                break;
            case CppType::CPPTYPE_FLOAT:
                reflection->SetRepeatedFloat(message, field, i, this->f);
                break;
            case CppType::CPPTYPE_DOUBLE:
                reflection->SetRepeatedDouble(message, field, i, this->d);
                break;
            default:
                LOG(ERROR)
                    << "Scalar::SetRepeated called on a non-scalar field: "
                    << field->name();
        }
    }
    static void SetRepeated(const google::protobuf::Reflection* reflection,
                            google::protobuf::Message* message,
                            const google::protobuf::FieldDescriptor* field,
                            const void* data, int n) {
        const char* ptr = (const char*)data;
        for (int i = 0; i < n; ++i) {
            switch (field->cpp_type()) {
                case CppType::CPPTYPE_INT32: {
                    int32_t val = *(const int32_t*)ptr;
                    reflection->SetRepeatedInt32(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                case CppType::CPPTYPE_INT64: {
                    int64_t val = *(const int64_t*)ptr;
                    reflection->SetRepeatedInt64(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                case CppType::CPPTYPE_UINT32: {
                    uint32_t val = *(const uint32_t*)ptr;
                    reflection->SetRepeatedUInt32(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                case CppType::CPPTYPE_UINT64: {
                    uint64_t val = *(const uint64_t*)ptr;
                    reflection->SetRepeatedUInt64(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                case CppType::CPPTYPE_FLOAT: {
                    float val = *(const float*)ptr;
                    reflection->SetRepeatedFloat(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                case CppType::CPPTYPE_DOUBLE: {
                    double val = *(const double*)ptr;
                    reflection->SetRepeatedDouble(message, field, i, val);
                    ptr += sizeof(val);
                    break;
                }
                default:
                    LOG(ERROR)
                        << "Scalar::GetRepeated called on a non-scalar field: "
                        << field->name();
            }
        }
    }

    void Add(const google::protobuf::Reflection* reflection,
             google::protobuf::Message* message,
             const google::protobuf::FieldDescriptor* field) const {
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_INT32:
                reflection->AddInt32(message, field, this->s32);
                break;
            case CppType::CPPTYPE_INT64:
                reflection->AddInt64(message, field, this->s64);
                break;
            case CppType::CPPTYPE_UINT32:
                reflection->AddUInt32(message, field, this->u32);
                break;
            case CppType::CPPTYPE_UINT64:
                reflection->AddUInt64(message, field, this->u64);
                break;
            case CppType::CPPTYPE_FLOAT:
                reflection->AddFloat(message, field, this->f);
                break;
            case CppType::CPPTYPE_DOUBLE:
                reflection->AddDouble(message, field, this->d);
                break;
            default:
                LOG(ERROR) << "Scalar::Add called on a non-scalar field: "
                           << field->name();
        }
    }
};

bool ProtoGui::DrawBoolField(const google::protobuf::FieldDescriptor* field) {
    const auto& options = field->options();
    bool rpt = field->is_repeated();
    auto* reflection = message_->GetReflection();
    int len = rpt ? reflection->FieldSize(*message_, field) : 1;
    bool changed = false;

    ImGui::PushID(field->number());
    for (int i = 0; i < len; ++i) {
        bool value;
        if (rpt) {
            ImGui::PushID(i);
            value = reflection->GetRepeatedBool(*message_, field, i);
        } else {
            value = reflection->GetBool(*message_, field);
        }

        std::string_view label = options.HasExtension(proto::label)
                                     ? options.GetExtension(proto::label)
                                     : field->name();
        if (columns_ == 0) {
            /* nothing */
        } else if (columns_ == -1) {
            ImGui::TableNextColumn();
            ImGui::TextUnformatted(label);
            ImGui::TableNextColumn();
            label = empty();
        } else {
        }

        if (ImGui::Checkbox(label, &value)) {
            changed = true;
            if (rpt) {
                reflection->SetRepeatedBool(message_, field, i, value);
            } else {
                reflection->SetBool(message_, field, value);
            }
        }
        if (rpt) {
            ImGui::PopID();
        }
    }
    ImGui::PopID();
    return changed;
}

constexpr Scalar ZERO = {0};
#define DRAW_SCALAR_FIELD(name_, setup_, func_, ...)                           \
    bool ProtoGui::DrawScalar##name_##Field(                                   \
        const google::protobuf::FieldDescriptor* field) {                      \
        const auto& options = field->options();                                \
        bool rpt = field->is_repeated();                                       \
        auto* reflection = message_->GetReflection();                          \
        int len = rpt ? reflection->FieldSize(*message_, field) : 1;           \
        bool changed = false;                                                  \
                                                                               \
        setup_                                                                 \
                                                                               \
            ImGui::PushID(field->number());                                    \
        if (n == 1) {                                                          \
            for (int i = 0; i < len; ++i) {                                    \
                Scalar value;                                                  \
                if (rpt) {                                                     \
                    ImGui::PushID(i);                                          \
                    value =                                                    \
                        Scalar::GetRepeated(reflection, message_, field, i);   \
                } else {                                                       \
                    value = Scalar::Get(reflection, message_, field);          \
                }                                                              \
                std::string_view label =                                       \
                    options.HasExtension(proto::label)                         \
                        ? options.GetExtension(proto::label)                   \
                        : field->name();                                       \
                if (columns_ == 0) {                                           \
                    /* nothing */                                              \
                } else if (columns_ == -1) {                                   \
                    ImGui::TableNextColumn();                                  \
                    ImGui::TextUnformatted(label);                             \
                    ImGui::TableNextColumn();                                  \
                    label = empty();                                           \
                } else {                                                       \
                }                                                              \
                if (func_(label, GetDataType(field), &value, ##__VA_ARGS__)) { \
                    changed = true;                                            \
                    if (rpt) {                                                 \
                        value.SetRepeated(reflection, message_, field, i);     \
                    } else {                                                   \
                        value.Set(reflection, message_, field);                \
                    }                                                          \
                }                                                              \
                if (rpt) {                                                     \
                    ImGui::PopID();                                            \
                }                                                              \
            }                                                                  \
        } else {                                                               \
            for (; len < n; ++len) {                                           \
                ZERO.Add(reflection, message_, field);                         \
            }                                                                  \
            Scalar value[n];                                                   \
            Scalar::GetRepeated(reflection, message_, field, value, n);        \
            std::string_view label = options.HasExtension(proto::label)        \
                                         ? options.GetExtension(proto::label)  \
                                         : field->name();                      \
            if (columns_ == 0) {                                               \
                /* nothing */                                                  \
            } else if (columns_ == -1) {                                       \
                ImGui::TableNextColumn();                                      \
                ImGui::TextUnformatted(label);                                 \
                ImGui::TableNextColumn();                                      \
                label = empty();                                               \
            } else {                                                           \
            }                                                                  \
            if (func_##N(label, GetDataType(field), &value, n,                 \
                         ##__VA_ARGS__)) {                                     \
                changed = true;                                                \
                Scalar::SetRepeated(reflection, message_, field, value, n);    \
            }                                                                  \
        }                                                                      \
        ImGui::PopID();                                                        \
        return changed;                                                        \
    }

// clang-format off
DRAW_SCALAR_FIELD(Drag,
    float speed = 1.0f;
    Scalar min;
    Scalar max;
    void* p_min = nullptr;
    void* p_max = nullptr;
    const char* format = nullptr;
    proto::ScalarDrag drag = options.GetExtension(proto::drag);
    int n = drag.n();
    if (n < 1) n = 1;
    if (drag.speed() != 0.0f) speed = drag.speed();
    if (drag.has_min()) {
        min = Scalar::From(drag.min());
        p_min = &min;
    }
    if (drag.has_max()) {
        max = Scalar::From(drag.max());
        p_max = &max;
    }
    if (!drag.format().empty()) {
        format = drag.format().c_str();
    }
    ,
    ImGui::DragScalar, speed, p_min, p_max, format, GetSliderFlags(drag.flags())
)

DRAW_SCALAR_FIELD(Slider,
    Scalar min;
    Scalar max;
    const char* format = nullptr;
    proto::ScalarSlider slider = options.GetExtension(proto::slider);
    int n = slider.n();
    if (n < 1) n = 1;
    min = Scalar::From(slider.min());
    max = Scalar::From(slider.max());
    if (!slider.format().empty()) {
        format = slider.format().c_str();
    }
    ,
    ImGui::SliderScalar, &min, &max, format, GetSliderFlags(slider.flags())
)

DRAW_SCALAR_FIELD(
    Input,
    Scalar step;
    Scalar fast;
    void* p_step = nullptr;
    void* p_fast = nullptr;
    const char* format = nullptr;
    int n = 1;
    ImGuiInputTextFlags flags = 0;
    if (options.HasExtension(proto::input)) {
        proto::ScalarInput input = options.GetExtension(proto::input);
        n = input.n();
        if (input.has_step()) {
            step = Scalar::From(input.step());
            p_step = &step;
        }
        if (input.has_fast()) {
            fast = Scalar::From(input.fast());
            p_fast = &fast;
        }
        if (!input.format().empty()) {
            format = input.format().c_str();
        }
        flags = GetInputTextFlags(input.flags());
    }
    if (n < 1) n = 1;
    ,
    ImGui::InputScalar, p_step, p_fast, format, flags
)
// clang-format on

bool ProtoGui::DrawStringField(const google::protobuf::FieldDescriptor* field) {
    const auto& options = field->options();
    bool rpt = field->is_repeated();
    auto* reflection = message_->GetReflection();
    int len = rpt ? reflection->FieldSize(*message_, field) : 1;
    bool changed = false;
    ImGuiInputTextFlags flags = 0;
    bool multiline = false;
    ImVec2 size(0, 0);
    if (options.HasExtension(proto::input_text)) {
        const auto& f = options.GetExtension(proto::input_text);
        flags = GetInputTextFlags(f);
        multiline = f.multiline();
        if (f.size_size() == 2) {
            size = ImVec2(f.size(0), f.size(1));
        }
    }
    ImGui::PushID(field->number());
    for (int i = 0; i < len; ++i) {
        std::string value;
        if (rpt) {
            ImGui::PushID(i);
            value = reflection->GetRepeatedString(*message_, field, i);
        } else {
            value = reflection->GetString(*message_, field);
        }
        std::string_view label = options.HasExtension(proto::label)
                                     ? options.GetExtension(proto::label)
                                     : field->name();
        if (columns_ == 0) {
            /* nothing */
        } else if (columns_ == -1) {
            ImGui::TableNextColumn();
            ImGui::TextUnformatted(label);
            ImGui::TableNextColumn();
            label = empty();
        } else {
        }
        bool ch;
        if (multiline) {
            ch = ImGui::InputTextMultiline(label, &value, size, flags);
        } else {
            ch = ImGui::InputText(label, &value, flags);
        }
        if (ch) {
            changed = true;
            if (rpt) {
                reflection->SetRepeatedString(message_, field, i, value);
            } else {
                reflection->SetString(message_, field, value);
            }
        }
        if (rpt) {
            ImGui::PopID();
        }
    }
    ImGui::PopID();
    return changed;
}

bool ProtoGui::DrawEnumField(const google::protobuf::FieldDescriptor* field) {
    const auto& options = field->options();
    bool rpt = field->is_repeated();
    auto* reflection = message_->GetReflection();
    int len = rpt ? reflection->FieldSize(*message_, field) : 1;
    bool changed = false;
    const auto* value = reflection->GetEnum(*message_, field);
    const auto* type = value->type();
    const char* items[type->value_count()];
    for (int i = 0; i < type->value_count(); ++i) {
        const auto* ev = type->value(i);
        items[i] = ev->name().c_str();
    }
    ImGui::PushID(field->number());
    for (int i = 0; i < len; ++i) {
        if (rpt) {
            ImGui::PushID(i);
            value = reflection->GetRepeatedEnum(*message_, field, i);
        } else {
            value = reflection->GetEnum(*message_, field);
        }
        std::string_view label = options.HasExtension(proto::label)
                                     ? options.GetExtension(proto::label)
                                     : field->name();
        if (columns_ == 0) {
            /* nothing */
        } else if (columns_ == -1) {
            ImGui::TableNextColumn();
            ImGui::TextUnformatted(label);
            ImGui::TableNextColumn();
            label = empty();
        } else {
        }
        int index = value->index();
        if (ImGui::Combo(label, &index, items, type->value_count())) {
            changed = true;
            if (rpt) {
                reflection->SetRepeatedEnum(message_, field, i,
                                            type->value(index));
            } else {
                reflection->SetEnum(message_, field, type->value(index));
            }
        }
        if (rpt) {
            ImGui::PopID();
        }
    }
    ImGui::PopID();
    return changed;
}

bool ProtoGui::DrawColorEditField(
    const google::protobuf::FieldDescriptor* field) {
    const auto& options = field->options();
    if (!field->is_repeated() || field->cpp_type() != CppType::CPPTYPE_FLOAT) {
        LOG(ERROR) << "color_edit fields must be repeated float fields";
        return false;
    }
    auto* reflection = message_->GetReflection();
    int len = reflection->FieldSize(*message_, field);
    const auto& flags = options.GetExtension(proto::color_edit);
    int n = (flags.type() == proto::ColorEditType::COLOR_EDIT_3 ||
             flags.type() == proto::ColorEditType::COLOR_PICKER_3)
                ? 3
                : 4;
    for (; len < n; ++len) {
        ZERO.Add(reflection, message_, field);
    }
    bool changed = false;

    ImGui::PushID(field->number());
    std::string_view label = options.HasExtension(proto::label)
                                 ? options.GetExtension(proto::label)
                                 : field->name();
    if (columns_ == 0) {
        /* nothing */
    } else if (columns_ == -1) {
        ImGui::TableNextColumn();
        ImGui::TextUnformatted(label);
        ImGui::TableNextColumn();
        label = empty();
    } else {
    }
    float color[n];
    Scalar::GetRepeated(reflection, message_, field, color, n);
    switch (flags.type()) {
        case proto::ColorEditType::COLOR_EDIT_3:
            changed = ImGui::ColorEdit3(label, color, GetColorEditFlags(flags));
            break;
        case proto::ColorEditType::COLOR_EDIT_4:
            changed = ImGui::ColorEdit4(label, color, GetColorEditFlags(flags));
            break;
        case proto::ColorEditType::COLOR_PICKER_3:
            changed =
                ImGui::ColorPicker3(label, color, GetColorEditFlags(flags));
            break;
        case proto::ColorEditType::COLOR_PICKER_4:
            changed =
                ImGui::ColorPicker4(label, color, GetColorEditFlags(flags));
            break;
        default:
            LOG(ERROR) << "Unknown color_edit.type: " << flags.type();
    }
    if (changed) {
        Scalar::SetRepeated(reflection, message_, field, color, n);
    }
    ImGui::PopID();
    return changed;
}

bool ProtoGui::Draw() {
    if (!valid_ || !message_) return false;
    bool changed = false;
    const auto* desc = message_->GetDescriptor();
    for (int i = 0; i < desc->field_count(); ++i) {
        const auto* field = desc->field(i);
        const auto& options = field->options();
        float item_width = options.GetExtension(proto::item_width);
        if (columns_) ImGui::TableNextRow();
        if (item_width != 0.0f) ImGui::PushItemWidth(item_width);
        switch (field->cpp_type()) {
            case CppType::CPPTYPE_BOOL:
                changed |= DrawBoolField(field);
                break;
            case CppType::CPPTYPE_INT32:
            case CppType::CPPTYPE_INT64:
            case CppType::CPPTYPE_UINT32:
            case CppType::CPPTYPE_UINT64:
            case CppType::CPPTYPE_FLOAT:
            case CppType::CPPTYPE_DOUBLE:
                if (options.HasExtension(proto::color_edit)) {
                    changed |= DrawColorEditField(field);
                } else if (options.HasExtension(proto::drag)) {
                    changed |= DrawScalarDragField(field);
                } else if (options.HasExtension(proto::slider)) {
                    changed |= DrawScalarSliderField(field);
                } else {
                    changed |= DrawScalarInputField(field);
                }
                break;
            case CppType::CPPTYPE_STRING:
                changed |= DrawStringField(field);
                break;
            case CppType::CPPTYPE_ENUM:
                changed |= DrawEnumField(field);
                break;
            case CppType::CPPTYPE_MESSAGE:
            default:
                LOG(ERROR) << field->name() << ": unsupported CppType "
                           << field->cpp_type();
        }
        if (item_width != 0.0f) ImGui::PopItemWidth();
    }
    return changed;
}

void ProtoGui::End() {
    if (!message_) return;
    const auto* desc = message_->GetDescriptor();
    const auto& begin = desc->options().GetExtension(proto::begin);
    if (default_width_ != 0.0f) ImGui::PopItemWidth();
    if (valid_ && desc->options().HasExtension(proto::table)) {
        ImGui::EndTable();
    }
    if (color_pushes_) ImGui::PopStyleColor(color_pushes_);
    switch (begin.begin_case()) {
        case proto::Begin::BeginCase::kWindow: {
            ImGui::End();
            break;
        }
        case proto::Begin::BeginCase::kChild: {
            ImGui::EndChild();
            break;
        }
        case proto::Begin::BeginCase::kGrp: {
            ImGui::EndGroup();
            break;
        }
        case proto::Begin::BeginCase::kPopup: {
            ImGui::EndPopup();
            break;
        }
        default:;
    }
    if (style_pushes_) ImGui::PopStyleVar(style_pushes_);
    valid_ = false;
}

}  // namespace gui
