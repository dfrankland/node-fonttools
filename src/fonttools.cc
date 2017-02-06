#include <node.h>
#include <node_buffer.h>
#include <Python/Python.h>

namespace fonttools {
  using v8::FunctionCallbackInfo;
  using v8::Isolate;
  using v8::Boolean;
  using v8::Value;
  using v8::Local;
  using v8::MaybeLocal;
  using v8::String;
  using v8::Object;

  PyObject *TTFont;

  PyObject *ConvertNodeBufferToPythonBuffer(const FunctionCallbackInfo<Value>& args) {
    PyObject *byteArray = PyByteArray_FromStringAndSize(node::Buffer::Data(args[0]), node::Buffer::Length(args[0]));
    PyObject *io = PyImport_ImportModule("io");
    PyObject *BytesIO = PyObject_GetAttrString(io, "BytesIO");
    PyObject *buffer = PyObject_CallObject(BytesIO, PyTuple_Pack(1, byteArray));
    PyObject *seek = PyObject_GetAttrString(buffer, "seek");
    PyObject_CallObject(seek, PyTuple_Pack(1, PyInt_FromLong(0)));
    return buffer;
  }

  Local<Object> ConvertPythonFileToNodeBuffer(const FunctionCallbackInfo<Value>& args, PyObject *file) {
    // Retrieve contents of "file" and size in bytes
    PyObject *getvalue = PyObject_GetAttrString(file, "getvalue");
    PyObject *value = PyObject_CallObject(getvalue, NULL);
    Py_ssize_t size = PyString_Size(value);
    char *data = PyString_AsString(value);

    // Close the "file"
    PyObject *close = PyObject_GetAttrString(file, "close");
    PyObject_CallObject(close, NULL);

    // Return Buffer
    MaybeLocal<Object> buffer = node::Buffer::Copy(args.GetIsolate(), data, size);
    return buffer.ToLocalChecked();
  }

  PyObject *GetInMemoryPythonFile(char *data = (char *)"") {
    PyObject *ioString = PyImport_ImportModule("StringIO");
    PyObject *StringIO = PyObject_GetAttrString(ioString, "StringIO");
    PyObject *file = PyObject_CallObject(StringIO, PyTuple_Pack(1, PyString_FromString(data)));
    PyObject *seek = PyObject_GetAttrString(file, "seek");
    PyObject_CallObject(seek, PyTuple_Pack(1, PyInt_FromLong(0)));
    return file;
  }

  void Decompile(const FunctionCallbackInfo<Value>& args) {
    // Get font Buffer
    PyObject *font = ConvertNodeBufferToPythonBuffer(args);

    // Create an empty "file"
    PyObject *file = GetInMemoryPythonFile();

    // Save XML to "file"
    PyObject *fontObject = PyObject_CallObject(TTFont, PyTuple_Pack(1, font));
    PyObject *saveXML = PyObject_GetAttrString(fontObject, "saveXML");
    PyObject_CallObject(saveXML, PyTuple_Pack(1, file));

    // Return a Buffer
    args.GetReturnValue().Set(ConvertPythonFileToNodeBuffer(args, file));
  }

  void Compile(const FunctionCallbackInfo<Value>& args) {
    // Get XML Buffer
    PyObject *xml = ConvertNodeBufferToPythonBuffer(args);

    // Create an empty "file"
    PyObject *file = GetInMemoryPythonFile();

    // Save binary font to "file"
    PyObject *fontObject = PyObject_CallObject(TTFont, NULL);
    PyObject *importXML = PyObject_GetAttrString(fontObject, "importXML");
    PyObject_CallObject(importXML, PyTuple_Pack(1, xml));
    PyObject *save = PyObject_GetAttrString(fontObject, "save");
    PyObject_CallObject(save, PyTuple_Pack(1, file));

    // Return a Buffer
    args.GetReturnValue().Set(ConvertPythonFileToNodeBuffer(args, file));
  }

  void SetFontToolsPath(const FunctionCallbackInfo<Value>& args) {
    // Get path of fonttools argument
    String::Utf8Value fontToolsPathObj(args[0]->ToString());
    char *fontToolsPath = *fontToolsPathObj;

    // Add current working directory to path
    PyObject *sys = PyImport_ImportModule("sys");
    PyObject *path = PyObject_GetAttrString(sys, "path");
    PyList_Insert(path, 0, PyString_FromString(fontToolsPath));

    // Get TTFont function
    PyObject *ttLib = PyImport_ImportModule("fontTools.ttLib");
    TTFont = PyObject_GetAttrString(ttLib, "TTFont");

    // Return methods
    Local<Object> obj = Object::New(args.GetIsolate());
    NODE_SET_METHOD(obj, "decompile", Decompile);
    NODE_SET_METHOD(obj, "compile", Compile);
    args.GetReturnValue().Set(obj);
  }

  void Init(Local<Object> exports, Local<Object> module) {
    // Initialize Python interpreter
    Py_Initialize();

    // Go to the future (needed for meaningful tracestacks etc.)
    PyImport_ImportModule("__future__");

    Isolate *isolate = exports->GetIsolate();
    Local<Object> obj = Object::New(isolate);
    obj->Set(String::NewFromUtf8(isolate, "value"), Boolean::New(isolate, true));
    exports->Set(String::NewFromUtf8(isolate, "__esModule"), obj);
    NODE_SET_METHOD(exports, "default", SetFontToolsPath);
  }

  NODE_MODULE(addon, Init);
}
