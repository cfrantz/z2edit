#ifndef EMPTY_PROJECT_UTIL_GUI_WIDGET_H
#define EMPTY_PROJECT_UTIL_GUI_WIDGET_H

#include <google/protobuf/message.h>
namespace gui {

class ProtoGui {
  public:
    static ProtoGui Begin(google::protobuf::Message* message, bool* p_open);
    bool Draw();
    void End();
    operator bool() const { return valid_; }
    ~ProtoGui() { End(); }

  private:
    bool DrawBoolField(const google::protobuf::FieldDescriptor* field);
    bool DrawScalarDragField(const google::protobuf::FieldDescriptor* field);
    bool DrawScalarSliderField(const google::protobuf::FieldDescriptor* field);
    bool DrawScalarInputField(const google::protobuf::FieldDescriptor* field);
    bool DrawStringField(const google::protobuf::FieldDescriptor* field);
    bool DrawEnumField(const google::protobuf::FieldDescriptor* field);
    bool DrawColorEditField(const google::protobuf::FieldDescriptor* field);

    ProtoGui() = default;
    ProtoGui(bool valid, google::protobuf::Message* message, int style_pushes,
             int color_pushes, float default_width)
        : valid_(valid),
          message_(message),
          style_pushes_(style_pushes),
          color_pushes_(color_pushes),
          default_width_(default_width) {}

    bool valid_ = false;
    google::protobuf::Message* message_ = nullptr;
    int style_pushes_ = 0;
    int color_pushes_ = 0;
    float default_width_ = 0.0f;
    int columns_ = 0;
};

}  // namespace gui
#endif  // EMPTY_PROJECT_UTIL_GUI_WIDGET_H
