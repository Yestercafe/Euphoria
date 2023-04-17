class UPoint : public UObject {
public:
    /**
     * %member
     * @desc
     * 点的 x 坐标
     */
    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    double X;
    /**
     * %member
     * @desc
     * 点的 y 坐标
     */
    UPROPERTY(EditAnywhere, BlueprintReadWrite)
    double Y;

public:

    /**
     * %method
     * @desc
     * 获取 x 坐标
     * @returns
     * x 坐标
     */
    double GetX() const;
    /**
     * %method
     * @desc
     * 获取 y 坐标
     * @returns
     * y 坐标
     */
    double GetY() const;

    /**
     * %method
     * @desc
     * 设置 x 坐标
     * 由参数给出
     *
     * @param X
     * 新给的 x 坐标
     *
     */
    void SetX(double X);
    /**
     * %method
     * @desc
     * 设置 y 坐标
     * 由参数给出
     * @param Y
     * 新给的 x 坐标
     */
    void SetY(double Y);

    /**
     * %method
     * @desc
     * 求两点距离
     * @param Rhs
     * 另一个点
     * @returns
     * 距离
     */
    double Distance(UPoint* Rhs) const;
};
