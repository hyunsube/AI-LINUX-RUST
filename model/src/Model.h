#include <onnxruntime_cxx_api.h>
#include <string>
#include <vector>

namespace AIModel::Model {

class ModelInstance {
    public:
        ModelInstance(const std::string &modelPath);
        
        void set_input_size(const std::vector<int64_t> &inputShape);
        void set_output_size(const std::vector<int64_t> &outputShape);

        void set_input_name(const std::string &inputName);
        void set_output_name(const std::string &outputName);

        std::string infernce(const std::vector<float> &inputData);
    
    private:

        std::string mModelPath;
        std::vector<int64_t> mInputShape;
        size_t mInputSize;
        std::vector<int64_t> mOutputShape;
        size_t mOutputSize;

        std::vector<const char*> mInputName = {"input"};
        char *mOutputName;

        Ort::Env mEnv;
        Ort::Session *mSession;
        Ort::SessionOptions mSessionOptions;
        Ort::AllocatorWithDefaultOptions mAllocators;
        float* mInputTensor;
        float* mOutputTensor;

        

};
}
