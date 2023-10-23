#ifndef UTIL_CONFIG_H
#define UTIL_CONFIG_H
#include <string>
#include <functional>

#include "google/protobuf/text_format.h"
#include "absl/log/log.h"
#include "util/file.h"
#include "util/os.h"

template<typename T>
class ConfigLoader {
  public:
    static ConfigLoader* Get() {
        static ConfigLoader<T> singleton;
        return &singleton;
    }
    static inline const T& GetConfig() { return Get()->config_; }
    static inline T* MutableConfig() { return &Get()->config_; }

    void Load(const std::string& filename,
              std::function<void(T*)> postprocess=nullptr) {
        filename_ = filename;
        postprocess_ = postprocess;
        Load(filename_, &config_);
        if (postprocess_)
            postprocess_(&config_);
    }
    void Parse(const std::string& data,
              std::function<void(T*)> postprocess=nullptr) {
        postprocess_ = postprocess;
        Load("", &config_, &data);
        if (postprocess_)
            postprocess_(&config_);
    }
    void Reload() {
        config_.Clear();
        Load(filename_, &config_);
        if (postprocess_)
            postprocess_(&config_);
    }
    inline const T& config() const { return config_; }

  protected:
    void Load(const std::string& filename, T* config,
              const std::string* data = nullptr) {
        std::string pb;
        std::string path = File::Dirname(filename);
        T local_config;

        if (data) {
            pb = *data;
        } else if (!File::GetContents(filename, &pb).ok()) {
            LOG(FATAL) << "Could not read '" << filename << "'.";
        }
        if (!google::protobuf::TextFormat::ParseFromString(pb, &local_config)) {
            LOG(FATAL) << "Could not parse '" << filename << "'.";
        }

        for(const auto& file : local_config.load()) {
            Load(os::path::Join({path, file}), &local_config);
        }

        config->MergeFrom(local_config);
    }

    ConfigLoader() {};
    T config_;
    std::string filename_;
    std::function<void(T*)> postprocess_;
};

#endif // UTIL_CONFIG_H
